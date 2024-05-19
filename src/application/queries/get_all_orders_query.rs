use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::order_repository::OrderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::ORDER;

/// ## Descripción
/// Función que obtiene todas las órdenes del repositorio.
///
/// ## Precondición
/// - Debe existir una instancia válida de `OrderRepository`.
///
/// ## Poscondición
/// - Se devuelve una respuesta JSON con el estado de éxito, el número de resultados y la lista de órdenes obtenidas del repositorio.

pub async fn get_all_orders_query() -> impl IntoResponse {
    let repository:OrderRepository = OrderRepository::new(ORDER);

    let mut query: Vec<Order> = Vec::new();
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