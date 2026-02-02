use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde_json::json;
use validator::Validate;

use crate::db::DbPool;
use crate::models::{CreateTaskRequest, Task, TaskResponse, UpdateTaskRequest, User};

/// GET /api/tasks/
pub async fn list_tasks(pool: web::Data<DbPool>, user: User) -> HttpResponse {
    let tasks: Vec<Task> =
        sqlx::query_as("SELECT * FROM tasks WHERE user_id = ? ORDER BY created_at DESC")
            .bind(user.id)
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();

    let response: Vec<TaskResponse> = tasks.into_iter().map(|t| t.into()).collect();
    HttpResponse::Ok().json(response)
}

/// POST /api/tasks/
pub async fn create_task(
    pool: web::Data<DbPool>,
    user: User,
    body: web::Json<CreateTaskRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({
            "detail": "Invalid request data.",
            "errors": e.to_string()
        }));
    }

    let now = Utc::now();
    let description = body.description.clone().unwrap_or_default();
    let completed = body.completed.unwrap_or(false);

    let result = sqlx::query(
        "INSERT INTO tasks (title, description, completed, created_at, updated_at, user_id) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&body.title)
    .bind(&description)
    .bind(completed)
    .bind(now)
    .bind(now)
    .bind(user.id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) => {
            let task_id = r.last_insert_rowid();
            HttpResponse::Created().json(TaskResponse {
                id: task_id,
                title: body.title.clone(),
                description,
                completed,
                created_at: now,
                updated_at: now,
                user: user.id,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to create task."
        })),
    }
}

/// GET /api/tasks/{id}/
pub async fn get_task(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
) -> HttpResponse {
    let task_id = path.into_inner();

    let task: Option<Task> =
        sqlx::query_as("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
            .bind(task_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    match task {
        Some(t) => HttpResponse::Ok().json(TaskResponse::from(t)),
        None => HttpResponse::NotFound().json(json!({
            "detail": "Task not found."
        })),
    }
}

/// PATCH /api/tasks/{id}/
pub async fn update_task(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
    body: web::Json<UpdateTaskRequest>,
) -> HttpResponse {
    let task_id = path.into_inner();

    // Check task exists and belongs to user
    let task: Option<Task> =
        sqlx::query_as("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
            .bind(task_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    let task = match task {
        Some(t) => t,
        None => {
            return HttpResponse::NotFound().json(json!({
                "detail": "Task not found."
            }));
        }
    };

    // Build update
    let title = body.title.clone().unwrap_or(task.title);
    let description = body.description.clone().unwrap_or(task.description);
    let completed = body.completed.unwrap_or(task.completed);
    let now = Utc::now();

    let result = sqlx::query(
        "UPDATE tasks SET title = ?, description = ?, completed = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&title)
    .bind(&description)
    .bind(completed)
    .bind(now)
    .bind(task_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(TaskResponse {
            id: task_id,
            title,
            description,
            completed,
            created_at: task.created_at,
            updated_at: now,
            user: user.id,
        }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to update task."
        })),
    }
}

/// DELETE /api/tasks/{id}/
pub async fn delete_task(
    pool: web::Data<DbPool>,
    user: User,
    path: web::Path<i64>,
) -> HttpResponse {
    let task_id = path.into_inner();

    // Check task exists and belongs to user
    let task: Option<Task> =
        sqlx::query_as("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
            .bind(task_id)
            .bind(user.id)
            .fetch_optional(pool.get_ref())
            .await
            .unwrap_or(None);

    if task.is_none() {
        return HttpResponse::NotFound().json(json!({
            "detail": "Task not found."
        }));
    }

    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(task_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "detail": "Failed to delete task."
        })),
    }
}
