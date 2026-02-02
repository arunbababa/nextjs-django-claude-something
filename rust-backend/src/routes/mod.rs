use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Health check
            .route("/health", web::get().to(health_check))
            // Auth routes (public)
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login))
                    .route("/logout", web::post().to(handlers::logout)),
            )
            // User routes (authenticated)
            .service(web::scope("/users").route("/me", web::get().to(handlers::get_current_user)))
            // Task routes (authenticated)
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(handlers::list_tasks))
                    .route("/", web::get().to(handlers::list_tasks))
                    .route("", web::post().to(handlers::create_task))
                    .route("/", web::post().to(handlers::create_task))
                    .route("/{id}", web::get().to(handlers::get_task))
                    .route("/{id}/", web::get().to(handlers::get_task))
                    .route("/{id}", web::patch().to(handlers::update_task))
                    .route("/{id}/", web::patch().to(handlers::update_task))
                    .route("/{id}", web::put().to(handlers::update_task))
                    .route("/{id}/", web::put().to(handlers::update_task))
                    .route("/{id}", web::delete().to(handlers::delete_task))
                    .route("/{id}/", web::delete().to(handlers::delete_task)),
            )
            // Debt routes (authenticated) - 貸し借り管理
            .service(
                web::scope("/debts")
                    .route("", web::get().to(handlers::list_debts))
                    .route("/", web::get().to(handlers::list_debts))
                    .route("", web::post().to(handlers::create_debt))
                    .route("/", web::post().to(handlers::create_debt))
                    .route("/summary", web::get().to(handlers::get_debt_summary))
                    .route("/summary/", web::get().to(handlers::get_debt_summary))
                    .route("/{id}", web::get().to(handlers::get_debt))
                    .route("/{id}/", web::get().to(handlers::get_debt))
                    .route("/{id}", web::patch().to(handlers::update_debt))
                    .route("/{id}/", web::patch().to(handlers::update_debt))
                    .route("/{id}", web::put().to(handlers::update_debt))
                    .route("/{id}/", web::put().to(handlers::update_debt))
                    .route("/{id}", web::delete().to(handlers::delete_debt))
                    .route("/{id}/", web::delete().to(handlers::delete_debt))
                    .route("/{id}/settle", web::post().to(handlers::settle_debt))
                    .route("/{id}/settle/", web::post().to(handlers::settle_debt)),
            ),
    );
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "backend": "rust"
    }))
}
