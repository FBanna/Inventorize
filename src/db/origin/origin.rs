use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Origin {
    pub origin_id: Uuid,
    pub name: String,
    pub url: String
}