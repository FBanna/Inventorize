use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportOrigin {
    pub name: String,
    pub url: Option<String>,
    pub price_py: Option<String>,
    pub py_pn: Option<String>,
    pub py_qr: Option<String>
}