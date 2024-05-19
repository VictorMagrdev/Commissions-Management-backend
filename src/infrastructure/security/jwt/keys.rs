use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

/// ## Descripción
/// Implementación del constructor para la estructura `Keys`, que se utiliza para crear nuevas instancias de `Keys` con claves de codificación y decodificación basadas en un secreto proporcionado.
///
/// ## Precondición
/// - Se proporciona un secreto válido como una referencia a una porción de bytes.
///
/// ## Poscondición
/// - Se devuelve una nueva instancia de `Keys` con las claves de codificación y decodificación correctamente inicializadas.
///

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
