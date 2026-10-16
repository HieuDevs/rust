// src/handlers/tasks.rs
// HTTP request handlers for task operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::db::tasks::TaskRepository;
use crate::error::{AppError, AppResult};
use crate::models::task::{CreateTask, Task, UpdateTask};

// AppState holds shared application state
// Clone is cheap because PgPool uses Arc internally
#[derive(Clone)]
pub struct AppState {
    pub task_repo: TaskRepository,
}

// GET /tasks - List all tasks
pub async fn list_tasks(State(state): State<AppState>) -> AppResult<Json<Vec<Task>>> {
    let tasks = state.task_repo.get_all().await?;
    Ok(Json(tasks))
}

// GET /tasks/{id} - Get a single task
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Task>> {
    let task = state.task_repo.get_by_id(id).await?;
    Ok(Json(task))
}

// POST /tasks - Create a new task
pub async fn create_task(
    State(state): State<AppState>,
    Json(input): Json<CreateTask>,
) -> AppResult<(StatusCode, Json<Task>)> {
    // Validate input
    if input.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title cannot be empty".to_string()));
    }

    let task = state.task_repo.create(input).await?;

    // Return 201 Created with the new task
    Ok((StatusCode::CREATED, Json(task)))
}

// PATCH /tasks/{id} - Update a task
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTask>,
) -> AppResult<Json<Task>> {
    // Validate title if provided
    if let Some(ref title) = input.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
    }

    let task = state.task_repo.update(id, input).await?;
    Ok(Json(task))
}

// DELETE /tasks/{id} - Delete a task
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = state.task_repo.delete(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("Task with id {} not found", id)))
    }
}