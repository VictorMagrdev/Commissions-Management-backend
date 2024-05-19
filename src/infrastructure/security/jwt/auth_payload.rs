use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use crate::domain::models::client::Client;
use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::client_repository::ClientRepository;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::tables::tables_name::{CLIENT, PROVIDER};
use crate::infrastructure::security::jwt::auth_body::AuthBody;
use crate::infrastructure::security::jwt::auth_error::AuthError;
use crate::infrastructure::security::jwt::claims::Claims;
use crate::KEYS;

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_email: String,
    client_type: String,
    client_password: String,
}

/// ## Descripción
/// Función asincrónica para manejar el acceso a áreas protegidas, utilizando las reclamaciones proporcionadas.
///
/// ## Precondición
/// - Se proporcionan reclamaciones válidas del usuario.
///
/// ## Poscondición
/// - Se devuelve un mensaje de bienvenida personalizado que incluye el nombre de usuario extraído de las reclamaciones.

pub async fn protected(claims: Claims) -> String {
    // Send the protected data to the user
    format!("Welcome to the protected area, {}!", claims.username)
}

/// ## Descripción
/// Función asincrónica para autorizar a un usuario mediante un proceso de autenticación basado en credenciales.
///
/// ## Precondición
/// - Se proporciona un objeto JSON `AuthPayload` válido que contiene las credenciales del cliente.
///
/// ## Poscondición
/// - Si la autenticación es exitosa, se genera un token de acceso válido y se devuelve en un cuerpo de autenticación JSON.
/// - Si la autenticación falla debido a credenciales faltantes o incorrectas, se devuelve un error `AuthError` apropiado.

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_email.is_empty() || payload.client_password.is_empty() || payload.client_type.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1)).and_utc().timestamp() as usize;

    if payload.client_type == "provider" {
        let repository: ProviderRepository = ProviderRepository::new(PROVIDER);
        let provedor_result = repository.get_by_mail(payload.client_email.clone()).await;
        match provedor_result {
            Ok(provedor) => {
                if &payload.client_email != provedor.get_email() || &payload.client_password != provedor.get_password() {
                    return Err(AuthError::WrongCredentials);
                }
                let id = provedor.get_id().ok_or(AuthError::MissingCredentials)?.id.to_string();
                let claims = Claims {
                    username: provedor.get_email().parse().map_err(|_| AuthError::WrongCredentials)?,
                    id: id.to_string(),
                    exp,
                };
                let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
                return Ok(Json(AuthBody::new(token)));
            },
            Err(_) => return Err(AuthError::WrongCredentials),
        }
    } else if payload.client_type == "client" {
        let repository: ClientRepository = ClientRepository::new(CLIENT);
        let cliente_result = repository.get_by_mail(payload.client_email.clone()).await;
        match cliente_result {
            Ok(cliente) => {
                if &payload.client_email != cliente.get_email() || &payload.client_password != cliente.get_password() {
                    return Err(AuthError::WrongCredentials);
                }
                let id = cliente.get_id().ok_or(AuthError::MissingCredentials)?.id.to_string();
                let claims = Claims {
                    username: cliente.get_email().parse().map_err(|_| AuthError::WrongCredentials)?,
                    id:id,
                    exp,
                };
                let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
                return Ok(Json(AuthBody::new(token)));
            },
            Err(_) => return Err(AuthError::WrongCredentials),
        }
    } else {
        return Err(AuthError::WrongCredentials);
    }
}
