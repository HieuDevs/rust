// src/tests.rs
// Test suite for task-api

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
};
use serde_json::Value;
use tower::ServiceExt;

use crate::error::AppError;
use crate::health_check;
use crate::models::task::{CreateTask, TaskStatus, UpdateTask};

#[tokio::test]
async fn test_health_check_endpoint() {
    let app = Router::new().route("/health", get(health_check));

    let request = Request::builder()
        .uri("/health")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(json["status"], "ok");
    assert!(json.get("timestamp").is_some());
    assert!(json["timestamp"].is_string());
}

#[tokio::test]
async fn test_health_check_method_not_allowed() {
    let app = Router::new().route("/health", get(health_check));

    let request = Request::builder()
        .uri("/health")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_not_found_route() {
    let app = Router::new().route("/health", get(health_check));

    let request = Request::builder()
        .uri("/non_existent_route")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_task_status_serialization() {
    let status = TaskStatus::InProgress;
    let serialized = serde_json::to_string(&status).unwrap();
    assert_eq!(serialized, "\"inprogress\"");

    let deserialized: TaskStatus = serde_json::from_str("\"inprogress\"").unwrap();
    assert_eq!(deserialized, TaskStatus::InProgress);

    let completed: TaskStatus = serde_json::from_str("\"completed\"").unwrap();
    assert_eq!(completed, TaskStatus::Completed);

    let pending: TaskStatus = serde_json::from_str("\"pending\"").unwrap();
    assert_eq!(pending, TaskStatus::Pending);
}

#[test]
fn test_create_task_deserialization() {
    let json = r#"{"title": "Learn Rust", "description": "Read The Book"}"#;
    let create_task: CreateTask = serde_json::from_str(json).unwrap();
    assert_eq!(create_task.title, "Learn Rust");
    assert_eq!(create_task.description, Some("Read The Book".to_string()));

    let json_no_desc = r#"{"title": "Buy groceries"}"#;
    let create_task_no_desc: CreateTask = serde_json::from_str(json_no_desc).unwrap();
    assert_eq!(create_task_no_desc.title, "Buy groceries");
    assert_eq!(create_task_no_desc.description, None);
}

#[test]
fn test_update_task_deserialization() {
    let json = r#"{"title": "Updated", "status": "completed"}"#;
    let update: UpdateTask = serde_json::from_str(json).unwrap();
    assert_eq!(update.title, Some("Updated".to_string()));
    assert_eq!(update.status, Some(TaskStatus::Completed));
    assert_eq!(update.description, None);
}

#[test]
fn test_app_error_into_response() {
    let not_found = AppError::NotFound("Item not found".to_string()).into_response();
    assert_eq!(not_found.status(), StatusCode::NOT_FOUND);

    let bad_request = AppError::BadRequest("Invalid data".to_string()).into_response();
    assert_eq!(bad_request.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_tasks_crud_flow() {
    dotenvy::dotenv().ok();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    let pool = match crate::create_pool(&database_url).await {
        Ok(pool) => pool,
        Err(_) => return,
    };

    let _ = sqlx::migrate!("./migrations").run(&pool).await;

    let app = crate::create_app(crate::AppState {
        task_repo: crate::TaskRepository::new(pool),
    });

    // 1. Create a task
    let create_payload = serde_json::json!({
        "title": "Integration Test Task",
        "description": "Created during integration test"
    });
    let request = Request::builder()
        .uri("/tasks")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&create_payload).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let created: Value = serde_json::from_slice(&body_bytes).unwrap();
    let task_id = created["id"].as_str().unwrap();
    assert_eq!(created["title"], "Integration Test Task");
    assert_eq!(created["status"], "pending");

    // 2. Get the created task
    let request = Request::builder()
        .uri(format!("/tasks/{}", task_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Update the task
    let update_payload = serde_json::json!({
        "status": "completed"
    });
    let request = Request::builder()
        .uri(format!("/tasks/{}", task_id))
        .method("PATCH")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&update_payload).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let updated: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(updated["status"], "completed");

    // 4. Delete the task
    let request = Request::builder()
        .uri(format!("/tasks/{}", task_id))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 5. Verify task is gone
    let request = Request::builder()
        .uri(format!("/tasks/{}", task_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_task_empty_title_validation() {
    dotenvy::dotenv().ok();
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };

    let pool = match crate::create_pool(&database_url).await {
        Ok(pool) => pool,
        Err(_) => return,
    };

    let app = crate::create_app(crate::AppState {
        task_repo: crate::TaskRepository::new(pool),
    });

    let payload = serde_json::json!({
        "title": "   ",
        "description": "should fail"
    });
    let request = Request::builder()
        .uri("/tasks")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
