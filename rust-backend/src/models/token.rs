use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Token {
    pub id: i64,
    pub key: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
}
