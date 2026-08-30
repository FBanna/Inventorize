use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportOrigin {
    pub name: String,
    pub url: Option<String>,
    pub hurl_get: Option<String>,
    pub hurl_price: Option<String>
}