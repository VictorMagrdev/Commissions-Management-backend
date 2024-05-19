use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use surrealdb::Error;
use surrealdb::err::Error::Thrown;
use crate::infrastructure::data::db_context::surreal_context::DB;

/// Representa un repositorio genérico para la gestión de registros.
pub struct Repository<T>  {
    pub table: String,
    _phantom: PhantomData<T>,
}

impl<T: for<'a> Deserialize<'a>  + Serialize> Repository<T> {
    /// Crea una nueva instancia del repositorio.
    ///
    /// ## Parámetros
    /// - `table`: El nombre de la tabla en la que se almacenarán los registros.
    pub fn new(table: &str) -> Self {
        Repository {
            table: String::from(table),
            _phantom: PhantomData,
        }
    }
    /// ## Descripción
    /// Método para obtener todos los registros de la tabla asociada al repositorio.
    /// ## Precondición
    /// - La conexión a la base de datos está establecida y es válida.
    /// - Se ha especificado correctamente la tabla asociada al repositorio.
    /// ## Poscondición
    /// - Se devuelve un vector que contiene todos los registros de la tabla.
    /// - Si la operación es exitosa, no se produce ningún cambio en el estado de la base de datos.
    pub async fn get_all(&self) -> Result<Vec<T>, Error> {
        let records: Vec<T> = DB.select(&self.table).await?;
        Ok(records)
    }

    /// ## Descripción
    /// Método para obtener un registro por su ID desde la tabla asociada al repositorio.
    ///
    /// ## Precondición
    /// - La conexión a la base de datos está establecida y es válida.
    /// - Se ha especificado correctamente la tabla asociada al repositorio.
    ///
    /// ## Poscondición
    /// - Si el registro con el ID proporcionado existe en la base de datos, se devuelve el registro correspondiente.
    pub async fn get_by_id(&self, id: String) -> Result<T, Error> {
        if let Some(record) = DB.select((&self.table, id.to_string())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("{} with id {} not found", self.table, id)));
        Err(error)
    }

    /// ## Descripción
    /// Método para crear un nuevo registro en la tabla asociada al repositorio.
    ///
    /// ## Precondición
    /// - La conexión a la base de datos está establecida y es válida.
    /// - Se ha especificado correctamente la tabla asociada al repositorio.
    /// - Se proporciona el contenido del registro que se desea crear.
    ///
    /// ## Poscondición
    /// - Si la creación del registro es exitosa, se devuelve un vector que contiene el registro creado.

    pub async fn create_repository(&self, content: T) -> Result<Vec<T>, Error> {
        let record: Vec<T> = DB.create(&self.table).content(content).await?;
        Ok(record)
    }

    /// ## Descripción
    /// Método para actualizar un registro existente en la tabla asociada al repositorio.
    ///
    /// ## Precondición
    /// - La conexión a la base de datos está establecida y es válida.
    /// - Se ha especificado correctamente la tabla asociada al repositorio.
    /// - Se proporciona un ID valido del registro que se desea actualizar.
    ///
    /// ## Poscondición
    /// - Si la actualización del registro es exitosa, se devuelve el registro actualizado.

    pub async fn update_repository(&self, id: &str, content: T) -> Result<T, Error> {
        let record = DB
            .update((&self.table, id.to_string()))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    /// ## Descripción
    /// Método para eliminar un registro por su ID de la tabla asociada al repositorio.
    ///
    /// ## Precondición
    /// - La conexión a la base de datos está establecida y es válida.
    /// - Se ha especificado correctamente la tabla asociada al repositorio.
    /// - Se proporciona un ID valido del registro que se desea eliminar.
    ///
    /// ## Poscondición
    /// - Si la eliminación del registro es exitosa, se devuelve el registro eliminado.

    pub async fn delete_repository(&self, id: String) -> Result<T, Error> {
        let result: T = DB.delete((&self.table, id.to_string())).await?.unwrap();
        Ok(result)
    }
}