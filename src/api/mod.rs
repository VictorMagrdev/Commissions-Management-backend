use axum::{Json, response::IntoResponse};
use serde_json::Value;

pub mod router;
/// ## Descripción
/// Cumple la labor de enrutador y manejador de rutas.
///
/// ## Poscondición
/// Una vez que la aplicación se despliegue, habrá un endpoint disponible que,
/// al ser consultado, devolverá un mensaje en formato JSON confirmando que la API está funcionando correctamente.
///
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Working fine, thanks!";

    let json_response:Value = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}