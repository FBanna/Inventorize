use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;





#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Label {
    pub label_id: Uuid,
    pub name: String,
    pub path: String
}