use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use validator::Validate;

use crate::db::DbPool;
use crate::models::{
    CreateDebtRequest, Debt, DebtResponse, DebtSummary, DebtType, UpdateDebtRequest, User,
};

/// GET /api/debts/
pub async fn list_debts(pool: web::Data<DbPool>, user: User) -> HttpResponse {
    let debts: Vec<Debt> =
        sqlx::query_as("SELECT * FROM debts WHERE user_id = ? ORDER BY created_at DESC")
            .bind(user.id)
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();

    let response: Vec<DebtResponse> = debts.into_iter().map(|d| d.into()).collect();
    HttpResponse::Ok().json(response)
}

/// POST /api/debts/
pub async fn create_debt(
    pool: web::Data<DbPool>,
    user: User,
    body: web::Json<CreateDebtRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Invalid request data.",
            "errors": e.to_string()
        }));
    }

    // Validate debt_type
    if DebtType::from_str(&body.debt_type).is_none() {
        return HttpResponse::BadRequest().json(json!({
            "debt_type": ["Must be 'lent' or 'borrowed'."]
        }));
    }

    let now = Utc::now();
    let description = body.description.clone().unwrap_or_default();

    let result = sqlx::query(
        "INSERT INTO debts (counterparty, amount, debt_type, description, is_settled, created_at, updated_at, user_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&body.counterparty)
    .bind(body.amount)
    .bind(&body.debt_type)
    .bind(&description)
    .bind(false)
    .bind(now)
    .bind(now)
    .bind(user.id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) => {
            let debt_id = r.last_insert_rowid();
            HttpResponse::Created().json(DebtResponse {
                id: debt_id,
                counterparty: body.counterparty.clone(),
                amount: body.amount,
                debt_type: body.debt_type.clone(),
                description,
                is_settled: false,
                created_at: now,
                updated_at: now,
                settled_at: None,
                user: user.id,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to create debt record."
        })),
    }
}

/// GET /api/debts/{id}/
pub async fn get_debt(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
) -> HttpResponse {
    let debt_id = path.into_inner();

    let debt: Option<Debt> =
        sqlx::query_as("SELECT * FROM debts WHERE id = ? AND user_id = ?")
            .bind(debt_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    match debt {
        Some(d) => HttpResponse::Ok().json(DebtResponse::from(d)),
        None => HttpResponse::NotFound().json(json!({
            "detail": "Debt record not found."
        })),
    }
}

/// PATCH /api/debts/{id}/
pub async fn update_debt(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
    body: web::Json<UpdateDebtRequest>,
) -> HttpResponse {
    let debt_id = path.into_inner();

    // Check debt exists and belongs to user
    let debt: Option<Debt> =
        sqlx::query_as("SELECT * FROM debts WHERE id = ? AND user_id = ?")
            .bind(debt_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    let debt = match debt {
        Some(d) => d,
        None => {
            return HttpResponse::NotFound().json(json!({
                "detail": "Debt record not found."
            }));
        }
    };

    // Validate debt_type if provided
    if let Some(ref dt) = body.debt_type {
        if DebtType::from_str(dt).is_none() {
            return HttpResponse::BadRequest().json(json!({
                "debt_type": ["Must be 'lent' or 'borrowed'."]
            }));
        }
    }

    // Build update
    let counterparty = body.counterparty.clone().unwrap_or(debt.counterparty);
    let amount = body.amount.unwrap_or(debt.amount);
    let debt_type = body.debt_type.clone().unwrap_or(debt.debt_type);
    let description = body.description.clone().unwrap_or(debt.description);
    let is_settled = body.is_settled.unwrap_or(debt.is_settled);
    let now = Utc::now();

    // Set settled_at if newly settled
    let settled_at = if is_settled && !debt.is_settled {
        Some(now)
    } else if !is_settled {
        None
    } else {
        debt.settled_at
    };

    let result = sqlx::query(
        "UPDATE debts SET counterparty = ?, amount = ?, debt_type = ?, description = ?, is_settled = ?, updated_at = ?, settled_at = ? WHERE id = ?",
    )
    .bind(&counterparty)
    .bind(amount)
    .bind(&debt_type)
    .bind(&description)
    .bind(is_settled)
    .bind(now)
    .bind(settled_at)
    .bind(debt_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(DebtResponse {
            id: debt_id,
            counterparty,
            amount,
            debt_type,
            description,
            is_settled,
            created_at: debt.created_at,
            updated_at: now,
            settled_at,
            user: user.id,
        }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to update debt record."
        })),
    }
}

/// DELETE /api/debts/{id}/
pub async fn delete_debt(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
) -> HttpResponse {
    let debt_id = path.into_inner();

    // Check debt exists and belongs to user
    let debt: Option<Debt> =
        sqlx::query_as("SELECT * FROM debts WHERE id = ? AND user_id = ?")
            .bind(debt_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    if debt.is_none() {
        return HttpResponse::NotFound().json(json!({
            "detail": "Debt record not found."
        }));
    }

    let result = sqlx::query("DELETE FROM debts WHERE id = ?")
        .bind(debt_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to delete debt record."
        })),
    }
}

/// POST /api/debts/{id}/settle/
pub async fn settle_debt(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
) -> HttpResponse {
    let debt_id = path.into_inner();

    // Check debt exists and belongs to user
    let debt: Option<Debt> =
        sqlx::query_as("SELECT * FROM debts WHERE id = ? AND user_id = ?")
            .bind(debt_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    let debt = match debt {
        Some(d) => d,
        None => {
            return HttpResponse::NotFound().json(json!({
                "detail": "Debt record not found."
            }));
        }
    };

    if debt.is_settled {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Debt is already settled."
        }));
    }

    let now = Utc::now();
    let result = sqlx::query(
        "UPDATE debts SET is_settled = ?, settled_at = ?, updated_at = ? WHERE id = ?",
    )
    .bind(true)
    .bind(now)
    .bind(now)
    .bind(debt_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(DebtResponse {
            id: debt_id,
            counterparty: debt.counterparty,
            amount: debt.amount,
            debt_type: debt.debt_type,
            description: debt.description,
            is_settled: true,
            created_at: debt.created_at,
            updated_at: now,
            settled_at: Some(now),
            user: user.id,
        }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to settle debt."
        })),
    }
}

/// GET /api/debts/summary/
pub async fn get_debt_summary(pool: web::Data<DbPool>, user: User) -> HttpResponse {
    // Get all unsettled debts
    let debts: Vec<Debt> = sqlx::query_as("SELECT * FROM debts WHERE user_id = ?")
        .bind(user.id)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    let mut total_lent: i64 = 0;
    let mut total_borrowed: i64 = 0;
    let mut unsettled_lent: i64 = 0;
    let mut unsettled_borrowed: i64 = 0;

    for debt in debts {
        match debt.debt_type.as_str() {
            "lent" => {
                total_lent += debt.amount;
                if !debt.is_settled {
                    unsettled_lent += debt.amount;
                }
            }
            "borrowed" => {
                total_borrowed += debt.amount;
                if !debt.is_settled {
                    unsettled_borrowed += debt.amount;
                }
            }
            _ => {}
        }
    }

    HttpResponse::Ok().json(DebtSummary {
        total_lent,
        total_borrowed,
        net_balance: total_lent - total_borrowed,
        unsettled_lent,
        unsettled_borrowed,
    })
}
