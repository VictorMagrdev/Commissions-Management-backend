use crate::domain::models::order::Order;
use crate::infrastructure::data::repositories::repository::Repository;

pub type OrderRepository = Repository<Order>;
