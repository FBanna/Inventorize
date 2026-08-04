use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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

    /// key: field name -- value: list of values
    pub facets: HashMap<String, Vec<SearchValue>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SearchValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}


