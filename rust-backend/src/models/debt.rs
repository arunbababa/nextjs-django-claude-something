use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// 貸し借りの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DebtType {
    Lent,     // 貸した（相手に貸している）
    Borrowed, // 借りた（相手から借りている）
}

impl DebtType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DebtType::Lent => "lent",
            DebtType::Borrowed => "borrowed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "lent" => Some(DebtType::Lent),
            "borrowed" => Some(DebtType::Borrowed),
            _ => None,
        }
    }
}

/// 貸し借り記録
#[derive(Debug, Clone, FromRow)]
pub struct Debt {
    pub id: i64,
    pub counterparty: String,      // 相手の名前
    pub amount: i64,               // 金額（円）
    pub debt_type: String,         // "lent" or "borrowed"
    pub description: String,       // メモ・理由
    pub is_settled: bool,          // 精算済みかどうか
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>, // 精算日時
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct DebtResponse {
    pub id: i64,
    pub counterparty: String,
    pub amount: i64,
    pub debt_type: String,
    pub description: String,
    pub is_settled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub user: i64,
}

impl From<Debt> for DebtResponse {
    fn from(debt: Debt) -> Self {
        DebtResponse {
            id: debt.id,
            counterparty: debt.counterparty,
            amount: debt.amount,
            debt_type: debt.debt_type,
            description: debt.description,
            is_settled: debt.is_settled,
            created_at: debt.created_at,
            updated_at: debt.updated_at,
            settled_at: debt.settled_at,
            user: debt.user_id,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDebtRequest {
    #[validate(length(min = 1, max = 100))]
    pub counterparty: String,
    #[validate(range(min = 1))]
    pub amount: i64,
    pub debt_type: String, // "lent" or "borrowed"
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDebtRequest {
    pub counterparty: Option<String>,
    pub amount: Option<i64>,
    pub debt_type: Option<String>,
    pub description: Option<String>,
    pub is_settled: Option<bool>,
}

/// サマリー情報
#[derive(Debug, Serialize)]
pub struct DebtSummary {
    pub total_lent: i64,           // 貸している総額
    pub total_borrowed: i64,       // 借りている総額
    pub net_balance: i64,          // 差し引き（正：貸し越し、負：借り越し）
    pub unsettled_lent: i64,       // 未精算の貸し
    pub unsettled_borrowed: i64,   // 未精算の借り
}
