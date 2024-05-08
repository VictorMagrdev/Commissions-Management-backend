use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::Error;

use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::client_repository::ClientRepository;
use crate::infrastructure::data::repositories::tables::tables_name::CLIENT;

pub async fn get_client_by_id_query(Path(id): Path<String>)
                                    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>
{
    let repository:ClientRepository = ClientRepository::new(CLIENT);
    let id:String = id.to_string();

    let todo: Result<Client, Error> = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}