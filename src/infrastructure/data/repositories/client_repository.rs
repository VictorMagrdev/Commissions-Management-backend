use surrealdb::err::Error::Thrown;
use surrealdb::Error;
use crate::domain::models::client::Client;
use crate::infrastructure::data::db_context::surreal_context::DB;
use crate::infrastructure::data::repositories::get_by_mail::GetByMail;
use crate::infrastructure::data::repositories::provider_repository::ProviderRepository;
use crate::infrastructure::data::repositories::repository::Repository;

/// Representa un repositorio para la gestión de clientes.
pub type ClientRepository = Repository<Client>;

impl GetByMail<Client> for ProviderRepository{
    async fn get_by_mail(&self, email: String) -> Result<Client, Error> {
        if let Some(record) = DB.select((&self.table, email.to_string())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("{} with email {} not found", self.table, email)));
        Err(error)
    }
}