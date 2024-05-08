use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::infrastructure::data::repositories::repository::Repository;

pub async fn delete_command<T:for<'a>Deserialize<'a>+ Serialize>(Path(id): Path<String>, table: &str)
                            -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    let repository:Repository<T> = Repository::new(table);
    let id:String = id.to_string();

    if let Ok(_) = repository.get_by_id(id.clone()).await {
        repository.delete_repository(id.clone()).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }

    let error_response:Value = serde_json::json!({
        "status": "error",
        "message": format!("{} with ID: {} not found", table, id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}