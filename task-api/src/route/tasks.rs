// src/routes/tasks.rs
// Route definitions for task endpoints

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::tasks::{
    create_task, delete_task, get_task, list_tasks, update_task, AppState,
};

// Build the task router with all endpoints
pub fn task_routes() -> Router<AppState> {
    Router::new()
        // Collection routes
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        // Individual resource routes
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}", patch(update_task))
        .route("/tasks/{id}", delete(delete_task))
}
