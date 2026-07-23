use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportOrigin {
    pub name: String,
    pub url: String
}