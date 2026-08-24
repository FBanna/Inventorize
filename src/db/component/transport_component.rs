use std::{collections::HashMap, fmt::Display, write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::db::component::properties::origin::origin::ComponentOrigin;





#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportComponent {
    pub class_instance_id: Uuid,

    pub name: String,
    pub stock: i32,
    pub manufacturer_id: Option<Uuid>,
    pub label_id: Option<Uuid>,


    // class_id, Json
    pub attributes: HashMap<Uuid, Json>, // looks like "attributes: {"(uuid)": "...", "(uuid)": "..."}"

    //pub origins: Vec<EmbeddedComponentOrigin>

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

impl Display for TransportComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "[ERROR] ClassError - AttributeParsing - Failed to parse class attributes: {}", template)
        return write!(f,
            "name: {}\nstock: {}\nclass_instance: {}\nattributes: {:#?}",
            self.name,
            self.stock,
            self.class_instance_id.as_hyphenated().to_string(),
            self.attributes
            
        );
    }
}



