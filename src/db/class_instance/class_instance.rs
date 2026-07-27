use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ClassInstance {
    pub class_instance_id: Uuid,
    pub class_id: Uuid,
    pub parent: Option<Uuid>
}

impl ClassInstance {
    pub fn to_tree(&self) -> {
        
    }
}