use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportOrigin {
    pub name: String,
    pub url: Option<String>,
    pub price_hurl: Option<String>,
    pub hurl_pn: Option<String>,
    pub hurl_qr: Option<String>
}