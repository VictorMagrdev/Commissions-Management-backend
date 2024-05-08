use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::Error;

use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

pub async fn get_provider_by_id_query(Path(id): Path<String>)
                                    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let repository:ProviderRepository = ProviderRepository::new(PROVIDER);
    let id:String = id.to_string();

    let todo: Result<Provider, Error> = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}