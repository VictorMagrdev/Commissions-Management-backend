use crate::domain::models::provider::Provider;
use crate::infrastructure::data::repositories::repository::Repository;

pub type ProviderRepository = Repository<Provider>;
