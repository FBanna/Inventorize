use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::db::component::properties::origin::origin::ComponentOrigin;





#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportComponent {
    pub class_instance_id: Uuid,

    pub name: String,
    pub stock: i32,
    pub manufacturer: Option<String>,
    pub label: Option<String>,

    pub attributes: Vec<EmbeddedComponentClassAttributes>,

    pub origins: Vec<EmbeddedComponentOrigin>

}
// add image, files, origins

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbeddedComponentClassAttributes {
    pub class_instance_id: Uuid,
    pub attributes: Json
}


// add image, files, origins
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmbeddedComponentOrigin {
    pub origin_id: Uuid,
    pub part_number: Option<String>,
    pub price: Option<i32>
}


