use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::application::commands::delete_command::delete_command;
use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::tables::tables_name::CLIENT;

/// ##Descripción
/// Función encargada de la eliminación de un cliente en el repositorio.
///
/// ## Precondición
/// -El id debe corresponder a un cliente existente.
///
/// ## Poscondición
/// -El cliente eliminado del repositorio.

pub async fn delete_client_command( Path(id): Path<String>)
                                -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {

    delete_command::<Client>(Path(id), &*CLIENT).await
}