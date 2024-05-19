use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::Value;

use crate::application::commands::delete_command::delete_command;
use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

/// ## Descripción
/// Función que elimina un proveedor del repositorio utilizando su ID.
///
/// ## Precondición
/// - Se debe proporcionar un ID válido del proveedor a eliminar.
/// - Debe existir una instancia de `ProviderRepository`.
///
/// ## Poscondición
/// - Si el proveedor con el ID proporcionado existe, se eliminará del repositorio.
/// - Si el proveedor no existe, se devolverá un mensaje de error indicando que no se encontró el proveedor.


pub async fn delete_provider_command( Path(id): Path<String>)
                                   -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    delete_command::<Provider>(Path(id),PROVIDER).await
}