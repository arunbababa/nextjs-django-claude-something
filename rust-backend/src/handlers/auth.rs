use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use rand::Rng;
use serde_json::json;
use validator::Validate;

use crate::db::DbPool;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, Token, User, UserResponse};

/// パスワードをハッシュ化
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// パスワードを検証
fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

/// トークンキーを生成
fn generate_token_key() -> String {
    let bytes: [u8; 20] = rand::thread_rng().gen();
    hex::encode(bytes)
}

/// パスワード強度を検証
fn validate_password(password: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if password.len() < 8 {
        errors.push("Password must be at least 8 characters long.".to_string());
    }

    if password.chars().all(|c| c.is_numeric()) {
        errors.push("Password can't be entirely numeric.".to_string());
    }

    let common_passwords = [
        "password", "12345678", "123456789", "qwerty", "abc123",
        "password1", "password123", "admin", "letmein", "welcome",
    ];
    if common_passwords.contains(&password.to_lowercase().as_str()) {
        errors.push("Password is too common.".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// ユーザー名を検証
fn validate_username(username: &str) -> Result<(), String> {
    let valid = username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '@' || c == '+' || c == '-');

    if valid {
        Ok(())
    } else {
        Err("Username may only contain letters, numbers, and @/./+/-/_ characters.".to_string())
    }
}

/// POST /api/auth/register
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Validate request
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Invalid request data.",
            "errors": e.to_string()
        }));
    }

    // Check password match
    if body.password != body.password2 {
        return HttpResponse::BadRequest().json(json!({
            "password2": ["Passwords don't match."]
        }));
    }

    // Validate username
    if let Err(e) = validate_username(&body.username) {
        return HttpResponse::BadRequest().json(json!({
            "username": [e]
        }));
    }

    // Validate password strength
    if let Err(errors) = validate_password(&body.password) {
        return HttpResponse::BadRequest().json(json!({
            "password": errors
        }));
    }

    // Check if username exists
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    if existing.is_some() {
        return HttpResponse::BadRequest().json(json!({
            "username": ["A user with that username already exists."]
        }));
    }

    // Check if email exists
    let existing: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    if existing.is_some() {
        return HttpResponse::BadRequest().json(json!({
            "email": ["A user with that email already exists."]
        }));
    }

    // Hash password
    let password_hash = match hash_password(&body.password) {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "detail": "Failed to create user."
            }));
        }
    };

    // Create user
    let now = Utc::now();
    let result = sqlx::query(
        "INSERT INTO users (username, email, password, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&body.username)
    .bind(&body.email)
    .bind(&password_hash)
    .bind(now)
    .bind(now)
    .execute(pool.get_ref())
    .await;

    let user_id = match result {
        Ok(r) => r.last_insert_rowid(),
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "detail": "Failed to create user."
            }));
        }
    };

    // Generate token
    let token_key = generate_token_key();
    let _ = sqlx::query("INSERT INTO tokens (key, user_id, created_at) VALUES (?, ?, ?)")
        .bind(&token_key)
        .bind(user_id)
        .bind(now)
        .execute(pool.get_ref())
        .await;

    HttpResponse::Created().json(AuthResponse {
        token: token_key,
        user: UserResponse {
            id: user_id,
            username: body.username.clone(),
            email: body.email.clone(),
        },
    })
}

/// POST /api/auth/login
pub async fn login(pool: web::Data<DbPool>, body: web::Json<LoginRequest>) -> HttpResponse {
    // Validate request
    if let Err(_) = body.validate() {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Invalid request data."
        }));
    }

    // Find user
    let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    let user = match user {
        Some(u) => u,
        None => {
            return HttpResponse::BadRequest().json(json!({
                "detail": "Unable to log in with provided credentials."
            }));
        }
    };

    // Verify password
    if !verify_password(&body.password, &user.password) {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Unable to log in with provided credentials."
        }));
    }

    // Get or create token
    let existing_token: Option<Token> = sqlx::query_as("SELECT * FROM tokens WHERE user_id = ?")
        .bind(user.id)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    let token_key = match existing_token {
        Some(t) => t.key,
        None => {
            let key = generate_token_key();
            let _ = sqlx::query("INSERT INTO tokens (key, user_id, created_at) VALUES (?, ?, ?)")
                .bind(&key)
                .bind(user.id)
                .bind(Utc::now())
                .execute(pool.get_ref())
                .await;
            key
        }
    };

    HttpResponse::Ok().json(AuthResponse {
        token: token_key,
        user: user.into(),
    })
}

/// POST /api/auth/logout
pub async fn logout(pool: web::Data<DbPool>, user: User) -> HttpResponse {
    let _ = sqlx::query("DELETE FROM tokens WHERE user_id = ?")
        .bind(user.id)
        .execute(pool.get_ref())
        .await;

    HttpResponse::Ok().json(json!({
        "detail": "Successfully logged out."
    }))
}
