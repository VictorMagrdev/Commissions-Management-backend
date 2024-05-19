use axum::{http::StatusCode, Json, response::IntoResponse};
use serde_json::Value;

use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::order_repository::OrderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::ORDER;

/// ##Descripción
/// Función encargada de la creación de un encargo en el repositorio.
///
/// ## Precondición
/// -El JSON debe ser valido, es decir, debe representar correctamente a un Encargo.
/// -Debe existir una instancia de OrderRepository
///
/// ## Poscondición
/// -Un JSON indicando la correcta creación de un encargo el cual ahora se encuentra en el repositorio.

pub async fn create_order_command(Json(body): Json<Order>)
                                     -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: OrderRepository = OrderRepository::new(ORDER);

    let order:Order = body.to_owned();

    let order:Order = repository.create_repository(order.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": order,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}