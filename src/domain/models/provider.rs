use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::domain::models::user::User;

/// ## Descripción
/// Tipo de usuario con las mismas capacidades que el cliente, con la diferencia de que
/// estos se han postulado y completado a un encargo publicado en el sistema.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Provider {
    id: Option<Thing>,
    user: User,
    rating: f32,
}