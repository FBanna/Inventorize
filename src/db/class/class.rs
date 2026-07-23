use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde_json::Value as Json;


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Class {
    pub class_id: Uuid,
    pub name: String,
    pub fields: Json,
    pub schema: Json
}