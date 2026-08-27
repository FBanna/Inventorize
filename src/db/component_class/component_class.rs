use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{Encode, prelude::{FromRow, Type}};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::db::component::component::ComponentWithAttributes;


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



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PagedComponentSearch {
    pub root: Option<Uuid>,
    pub units: Vec<UnitComponentClassSearch>,
    pub state: TablePageQuery
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FacetSearch {
    pub root: Option<Uuid>,
    pub units: Vec<UnitComponentClassSearch>,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TablePageQuery {
    pub page_pos: i32,
    pub page_size: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PagedComponentSearchResult {
    pub results: Vec<ComponentWithAttributes>,
    pub has_next: bool
}

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