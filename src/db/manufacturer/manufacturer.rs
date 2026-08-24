use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Manufacturer {
    pub manufacturer_id: Uuid,
    pub name: String,
    pub url: Option<String>
}