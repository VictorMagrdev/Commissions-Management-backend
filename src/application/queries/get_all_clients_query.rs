use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::client_repository::ClientRepository;
use crate::infrastructure::data::repositories::tables::tables_name::CLIENT;

pub async fn get_all_clients_query() -> impl IntoResponse {
    let repository:ClientRepository = ClientRepository::new(CLIENT);

    let mut query: Vec<Client> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        query = result;
    }

    let json_response:Value = serde_json::json!({
        "status": "success",
        "results": query.len(),
        "query": query,
    });

    Json(json_response)
}