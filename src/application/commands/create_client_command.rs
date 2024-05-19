use axum::{http::StatusCode, Json, response::IntoResponse};
use serde_json::Value;

use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::client_repository::ClientRepository;
use crate::infrastructure::data::repositories::tables::tables_name::CLIENT;

/// ##Descripción
/// Función encargada de la creación de un cliente en el repositorio.
///
/// ## Precondición
/// -El JSON debe ser valido, es decir, debe representar correctamente a un cliente.
/// -Debe existir una instancia de ClientRepository
///
/// ## Poscondición
/// -Un JSON indicando la correcta creación de un cliente el cual ahora se encuentra en el repositorio.

pub async fn create_client_command(Json(body): Json<Client>)
                                 -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let repository: ClientRepository = ClientRepository::new(CLIENT);

    let client:Client = body.to_owned();

    let client: Client = repository.create_repository(client.clone()).await.unwrap()[0].to_owned();

    let json_response: Value = serde_json::json!({
        "status": "success".to_string(),
        "data": client,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}