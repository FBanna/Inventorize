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

    pub attributes: Vec<TransportComponentClassAttributes>,
    
    pub origin: ComponentOrigin

}

// add image, files, origins
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportComponentClassAttributes {
    pub class_instance_id: Uuid,
    pub attributes: Json
}