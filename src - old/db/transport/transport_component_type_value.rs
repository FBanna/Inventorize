// use sqlx::Value as Json;

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;


#[derive(Serialize, Deserialize)]
pub struct TransportComponentTypeValue {

    pub type_id: i64,
    pub attributes: Json

}