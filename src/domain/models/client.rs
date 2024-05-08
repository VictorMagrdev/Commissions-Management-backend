use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Client {
    id: Option<Thing>,
    user: User
}