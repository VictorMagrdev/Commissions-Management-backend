use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::Error;

use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::order_repository::OrderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::ORDER;

/// ## Descripción
/// Función que obtiene una orden del repositorio por su ID.
///
/// ## Precondición
/// - Se debe proporcionar un ID válido de la orden.
/// - Debe existir una instancia válida de `OrderRepository`.
///
/// ## Poscondición
/// - Si la orden con el ID proporcionado existe, se devuelve una respuesta con el estado `200 OK` y los datos de la orden en formato JSON.
///

pub async fn get_order_by_id_query(Path(id): Path<String>)
                                    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let repository:OrderRepository = OrderRepository::new(ORDER);
    let id:String = id.to_string();

    let todo: Result<Order, Error> = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}