use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::Error;

use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::PROVIDER;

/// ## Descripción
/// Función que obtiene un proveedor del repositorio por su ID.
///
/// ## Precondición
/// - Se debe proporcionar un ID válido del proveedor.
/// - Debe existir una instancia válida de `ProviderRepository`.
///
/// ## Poscondición
/// - Si el proveedor con el ID proporcionado existe, se devuelve una respuesta con el estado `200 OK` y los datos del proveedor en formato JSON.
///

pub async fn get_provider_by_id_query(Path(id): Path<String>)
                                    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let repository:ProviderRepository = ProviderRepository::new(PROVIDER);
    let id:String = id.to_string();

    let provider: Result<Provider, Error> = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(provider)));
}