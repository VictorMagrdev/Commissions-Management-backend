use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::repository::Repository;

pub type ClientRepository = Repository<Client>;

