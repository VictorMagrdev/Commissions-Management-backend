use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    phone: String,
    email: String,
}


