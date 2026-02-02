use actix_web::{dev::Payload, error::ErrorUnauthorized, http::header, FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use serde_json::json;

use crate::db::DbPool;
use crate::models::User;

/// User extractor for authenticated routes
/// Extracts user from token in Authorization header
impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Get pool from app data
        let pool = match req.app_data::<actix_web::web::Data<DbPool>>() {
            Some(p) => p.clone(),
            None => {
                return futures::future::err(ErrorUnauthorized(
                    json!({"detail": "Internal server error."}),
                ))
            }
        };

        // Get Authorization header
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(h) => h,
            None => {
                return futures::future::err(ErrorUnauthorized(
                    json!({"detail": "Authentication credentials were not provided."}),
                ))
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => {
                return futures::future::err(ErrorUnauthorized(
                    json!({"detail": "Invalid token header."}),
                ))
            }
        };

        // Parse "Token <key>" format
        let parts: Vec<&str> = auth_str.splitn(2, ' ').collect();
        if parts.len() != 2 || parts[0] != "Token" {
            return futures::future::err(ErrorUnauthorized(
                json!({"detail": "Invalid token header. No credentials provided."}),
            ));
        }

        let token_key = parts[1].to_string();
        let pool_inner = pool.get_ref().clone();

        // We need to block here since FromRequest doesn't support async directly
        // In a real application, you might want to use a more sophisticated approach
        let user = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Find token
                let token: Option<crate::models::Token> =
                    sqlx::query_as("SELECT * FROM tokens WHERE key = ?")
                        .bind(&token_key)
                        .fetch_optional(&pool_inner)
                        .await
                        .unwrap_or(None);

                if let Some(t) = token {
                    // Find user
                    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE id = ?")
                        .bind(t.user_id)
                        .fetch_optional(&pool_inner)
                        .await
                        .unwrap_or(None);
                    user
                } else {
                    None
                }
            })
        })
        .join()
        .unwrap_or(None);

        match user {
            Some(u) => ok(u),
            None => futures::future::err(ErrorUnauthorized(json!({"detail": "Invalid token."}))),
        }
    }
}
