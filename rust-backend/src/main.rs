mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

use config::Config;
use db::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();

    // Initialize logger
    if config.debug {
        std::env::set_var("RUST_LOG", "actix_web=debug,rust_backend=debug");
    } else {
        std::env::set_var("RUST_LOG", "actix_web=info,rust_backend=info");
    }
    env_logger::init();

    // Initialize database
    let pool = init_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    let cors_origins = config.cors_origins.clone();
    let host = config.host.clone();
    let port = config.port;

    log::info!("Starting Rust backend server on {}:{}", host, port);
    log::info!("Debug mode: {}", config.debug);
    log::info!("CORS allowed origins: {:?}", cors_origins);

    println!("\n========================================");
    println!("  Rust Backend Server");
    println!("========================================");
    println!("\nServer running at http://{}:{}", host, port);
    println!("\nAPI Endpoints:");
    println!("  POST /api/auth/register     - Register new user");
    println!("  POST /api/auth/login        - Login and get token");
    println!("  POST /api/auth/logout       - Logout (requires auth)");
    println!("  GET  /api/users/me          - Get current user (requires auth)");
    println!("  GET  /api/tasks/            - List tasks (requires auth)");
    println!("  POST /api/tasks/            - Create task (requires auth)");
    println!("  GET  /api/tasks/:id/        - Get task (requires auth)");
    println!("  PATCH /api/tasks/:id/       - Update task (requires auth)");
    println!("  DELETE /api/tasks/:id/      - Delete task (requires auth)");
    println!("  GET  /api/debts/            - List debt records (requires auth)");
    println!("  POST /api/debts/            - Create debt record (requires auth)");
    println!("  GET  /api/debts/:id/        - Get debt record (requires auth)");
    println!("  PATCH /api/debts/:id/       - Update debt record (requires auth)");
    println!("  DELETE /api/debts/:id/      - Delete debt record (requires auth)");
    println!("  POST /api/debts/:id/settle/ - Mark debt as settled (requires auth)");
    println!("  GET  /api/debts/summary/    - Get debt summary (requires auth)");
    println!("  GET  /api/health            - Health check");
    println!("\n========================================\n");

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _req_head| {
                cors_origins
                    .iter()
                    .any(|allowed| origin.as_bytes() == allowed.as_bytes())
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ORIGIN,
                header::HeaderName::from_static("x-requested-with"),
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind((host, port))?
    .run()
    .await
}
