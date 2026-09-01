use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentOrigin {
    pub origin_id: Uuid,
    pub component_id: Uuid,
    pub part_number: Option<String>,
    pub price: Option<i32>
}
