use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::repository::Repository;

/// Representa un repositorio para la gestión de encargos.
pub type OrderRepository = Repository<Order>;
