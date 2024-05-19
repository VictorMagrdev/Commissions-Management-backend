use axum::{Router, routing::get};
use axum::routing::post;

use crate::application::commands::create_client_command::create_client_command;
use crate::application::commands::create_order_command::create_order_command;
use crate::application::commands::create_provider_command::create_provider_command;
use crate::application::queries::get_all_clients_query::get_all_clients_query;
use crate::application::queries::get_all_orders_query::get_all_orders_query;
use crate::application::queries::get_all_providers_query::get_all_providers_query;

use super::health_checker_handler;

/// ## Descripción
/// Configura y devuelve un objeto tipo Router con las rutas. Emplea health_checker_handler
/// para realizar la verificación de estado.
///
/// ## Poscondición
/// Se crea un enrutador con ruta definida para la verificación de estado. Ademas,
/// debe dar un respuesta JSON confirmando el buen funcionamiento de la API.
///
pub fn create_router() -> Router {
    Router::new()
        .route(
            "/api/healthchecker",
            get(health_checker_handler)
        )
        .route(
            "/api/clients",
            post(create_client_command)
                .get(get_all_clients_query),
        )
        .route("/api/orders",
            get(get_all_orders_query)
                .post(create_order_command)
        )
        .route("/api/providers",
               get(get_all_providers_query)
                   .post(create_provider_command)
        )
}