use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use surrealdb::Error;
use surrealdb::err::Error::Thrown;
use crate::infrastructure::data::db_context::surreal_context::DB;

pub struct Repository<T>  {
    pub table: String,
    _phantom: PhantomData<T>,
}

impl<T: for<'a> Deserialize<'a>  + Serialize> Repository<T> {
    pub fn new(table: &str) -> Self {
        Repository {
            table: String::from(table),
            _phantom: PhantomData,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<T>, Error> {
        let records: Vec<T> = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<T, Error> {
        if let Some(record) = DB.select((&self.table, id.to_string())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("{} with id {} not found", self.table, id)));
        Err(error)
    }

    pub async fn create_repository(&self, content: T) -> Result<Vec<T>, Error> {
        let record: Vec<T> = DB.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update_repository(&self, id: &str, content: T) -> Result<T, Error> {
        let record = DB
            .update((&self.table, id.to_string()))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete_repository(&self, id: String) -> Result<T, Error> {
        let result: T = DB.delete((&self.table, id.to_string())).await?.unwrap();
        Ok(result)
    }
}