use serde::{Deserialize, Serialize};
use uuid::Uuid;





#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportLabel {
    pub name: String,
    pub path: String
}

