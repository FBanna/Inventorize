use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Encode, prelude::{FromRow, Type}};
use uuid::Uuid;
use serde_json::Value as Json;


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentClass {
    pub component_id: Uuid,
    pub class_instance_id: Uuid,
    pub attributes: Json
}



///     [
///         {
///             class_instance_id: UUID,
///             facets: {
///                 "resistance": [60, 120],
///                 "package": ["0402"]
///             }
///         },
///         {
///             ...
///         }
/// 
///     ]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentClassSearch {
    pub class_instance_id: Uuid,
    pub facets: HashMap<String, Vec<Json>>
}