use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;


#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct Origin {
    pub origin_id: Uuid,
    pub name: String,
    pub url: Option<String>,
    pub hurl_get: Option<String>,
    pub hurl_price: Option<String>
}