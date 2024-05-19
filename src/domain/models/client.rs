use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::domain::models::user::User;

/// ## Descripción
/// Representación de los usuarios de tipo cliente, quienes
/// podran interactuar con el sistema gestionando encargos
/// y acciones relacionadas a estos mismos.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    id: Option<Thing>,
    user: User
}

impl Client {
    pub fn get_email(&self) -> &str {
        &self.user.get_email()
    }
    pub fn get_password(&self) -> &str {
        &self.user.get_password()
    }

    pub fn get_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}