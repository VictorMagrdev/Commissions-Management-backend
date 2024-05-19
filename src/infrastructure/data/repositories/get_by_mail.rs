use surrealdb::Error;

pub trait GetByMail<T>{
    async fn get_by_mail(&self, email: String) -> Result<T, Error>;
}