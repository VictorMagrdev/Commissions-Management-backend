use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::repository::Repository;

/// Representa un repositorio para la gestión de proveedores.
pub type ProviderRepository = Repository<Provider>;
