use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use serde::Deserialize;
use crate::infrastructure::security::jwt::auth_body::AuthBody;
use crate::infrastructure::security::jwt::auth_error::AuthError;
use crate::infrastructure::security::jwt::claims::Claims;
use crate::KEYS;

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

/// ## Descripción
/// Función asincrónica para manejar el acceso a áreas protegidas, utilizando las reclamaciones proporcionadas.
///
/// ## Precondición
/// - Se proporcionan reclamaciones válidas del usuario.
///
/// ## Poscondición
/// - Se devuelve un mensaje de bienvenida personalizado que incluye el nombre de usuario extraído de las reclamaciones.

async fn protected(claims: Claims) -> String {
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

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here, basic verification is used, but normally you would use a database
    if &payload.client_id != "foo" || &payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    // create the timestamp for the expiry time - here the expiry time is 1 day
    // in production you may not want to have such a long JWT life
    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1)).and_utc().timestamp() as usize;
    let claims = Claims {
        username: payload.client_id,
        exp,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(
        |_| AuthError::TokenCreation
    )?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
