use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportClassInstance {
    pub class_id: Uuid,
    pub parent: Option<Uuid>
}