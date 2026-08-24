use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportManufacturer {
    pub name: String,
    pub url: Option<String>
}