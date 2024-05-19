use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

/// ## Descripción
/// Implementación de un constructor para la estructura `AuthBody`, utilizado para crear una nueva instancia de `AuthBody` con los datos proporcionados.
/// ## Precondición
/// - Se proporciona un token de acceso válido.
///
/// ## Poscondición
/// - Se devuelve una nueva instancia de `AuthBody` con el token de acceso y el tipo de token predeterminado inicializados.



impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
