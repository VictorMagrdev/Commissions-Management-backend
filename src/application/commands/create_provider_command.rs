use axum::{http::StatusCode, Json, response::IntoResponse};
use serde_json::Value;

use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

pub async fn create_provider_command(Json(body): Json<Provider>)
                                   -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: ProviderRepository = ProviderRepository::new(PROVIDER);

    let provider:Provider = body.to_owned();
    let provider:Provider = repository.create_repository(provider.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": provider,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}