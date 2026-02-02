use actix_web::HttpResponse;

use crate::models::{User, UserResponse};

/// GET /api/users/me
pub async fn get_current_user(user: User) -> HttpResponse {
    HttpResponse::Ok().json(UserResponse::from(&user))
}
