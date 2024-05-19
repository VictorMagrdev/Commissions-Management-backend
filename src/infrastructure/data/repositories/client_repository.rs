use crate::domain::models::client::Client;
use crate::infrastructure::data::repositories::repository::Repository;

/// Representa un repositorio para la gestión de clientes.
pub type ClientRepository = Repository<Client>;

