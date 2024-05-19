use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

/// ## Descripción
/// Función que establece la conexión con la base de datos.
///
/// ## Precondición
/// - La base de datos debe estar accesible en la dirección y puerto especificados.
/// - Se debe poder iniciar sesión con un usuario administrador.
///
/// ## Poscondición
/// - Si la conexión y la autenticación son exitosas, se selecciona la base de datos `katheryne`.
/// - Si ocurre algún error durante la conexión o la autenticación, se devuelve un error.


pub async fn connect_db() -> Result<()> {
    let _ = DB.connect::<Ws>("localhost:8000").await?;
    let _ = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await;
    let _ = DB.use_ns("katheryne").use_db("katheryne").await?;
    Ok(())
}