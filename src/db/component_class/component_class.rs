use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde_json::Value as Json;


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentClass {
    pub component_id: Uuid,
    pub class_instance_id: Uuid,
    pub attributes: Json
}