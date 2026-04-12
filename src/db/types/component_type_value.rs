use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentTypeValue {
    pub component_id: i64,
    pub type_id: i64,
    pub attributes: JsonValue
}