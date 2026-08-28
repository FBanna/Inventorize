use std::{collections::HashMap, fmt::Display, write};

use async_trait::async_trait;
use axum::body::Bytes;
use axum_typed_multipart::{FieldMetadata, TryFromChunks, TryFromField, TryFromMultipart, TypedMultipartError};
use futures::Stream;
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
            "name: {}\nstock: {}\nclass_instance: {}\nlabel: {}\nmanufacturer: {}\nattributes: {:#?}",
            self.name,
            self.stock,
            self.class_instance_id.as_hyphenated().to_string(),
            self.label_id.unwrap_or_default().as_hyphenated().to_string(),
            self.manufacturer_id.unwrap_or_default().as_hyphenated().to_string(),
            self.attributes
            
        );
    }
}

#[async_trait]
impl TryFromChunks for TransportComponent {

    async fn try_from_chunks(
        mut chunks: impl 'async_trait
            + Stream<Item = Result<Bytes, TypedMultipartError>>
            + Send
            + Sync
            + Unpin,
        metadata: FieldMetadata,
    ) -> Result<Self, TypedMultipartError> {
        let bytes = Bytes::try_from_chunks(chunks, metadata).await?;

        serde_json::from_slice(&bytes).map_err(|e| TypedMultipartError::Other { source: e.into() })
    }

}

