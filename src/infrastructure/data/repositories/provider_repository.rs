use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::repository::Repository;
use surrealdb::err::Error::Thrown;
use crate::infrastructure::data::db_context::surreal_context::DB;
use surrealdb::Error;
use crate::infrastructure::data::repositories::get_by_mail::GetByMail;

/// Representa un repositorio para la gestión de proveedores.
pub type ProviderRepository = Repository<Provider>;
impl GetByMail<Provider> for ProviderRepository{
    async fn get_by_mail(&self, email: String) -> Result<Provider, Error> {
        if let Some(record) = DB.select((&self.table, email.to_string())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("{} with email {} not found", self.table, email)));
        Err(error)
    }
}