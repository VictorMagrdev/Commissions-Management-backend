use serde::{Deserialize, Serialize};

/// ## Descripción
/// Superclase de la cual se pueden crear usuarios tipo cliente, proveedor y administrador,

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    phone: String,
    email: String,
}


