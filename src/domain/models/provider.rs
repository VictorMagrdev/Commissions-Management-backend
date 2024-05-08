use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Provider {
    id: Option<Thing>,
    user: User,
    rating: f32,
}