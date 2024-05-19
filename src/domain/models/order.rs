use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

/// ##Descripción
/// Representación de los que se podran realizar por parte de los usuarios,
/// la información representa una necesidad de quien la creó.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Category {
    Remote,
    InPerson,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Status {
    Pending,
    Accepted,
    InProgress,
    Finished,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    id: Option<Thing>,
    title: String,
    status: Status,
    description: String,
    payment: f32,
    user: Id,
    creation_date: Option<DateTime<Local>>,
    end_date: Option<DateTime<Local>>,
    availability_date: Option<DateTime<Local>>,
    category: Category
}
