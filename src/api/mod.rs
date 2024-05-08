use axum::{Json, response::IntoResponse};
use serde_json::Value;

pub mod router;

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response:Value = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}