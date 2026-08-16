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
///             facets: [
///                 "resistance": [60, 120],
///                 "package": ["0402"]
///             ]
///         },
///         {
///             ...
///         }
/// 
///     ]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitComponentClassSearch {
    pub class_instance_id: Uuid,
    pub facets: HashMap<String, Vec<Json>>
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SearchFacet {
//     pub key: String,
//     pub values: Vec<Json>
// }


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentSearch {
    pub root: Option<Uuid>,
    pub units: Vec<UnitComponentClassSearch>
}

// pub struct UnitComponentClassFacet {
//     pub class_instance_id: Uuid,
//     pub facets: HashMap<String, Vec<Json>>
// }

// [
//
//     {
//         class_instance_id: UUID,
//         facets: {
//             "resistance": [
//                 {"value": ..., "count": ...},
//                 {...}
//             ]
//         }
//     },
//     {
//         ...
//     }
//
// ]
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct SearchFacets {
    pub jsonb_agg: Option<Json>
}