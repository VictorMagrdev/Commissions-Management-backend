use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::application::commands::delete_command::delete_command;
use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

pub async fn delete_provider_command( Path(id): Path<String>)
                                   -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    delete_command::<Provider>(Path(id),PROVIDER).await
}