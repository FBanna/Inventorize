use std::{fs, io::Cursor, path::{Path, PathBuf}};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Execute, Pool, QueryBuilder, Row, Postgres, PgPool, migrate::{MigrateDatabase, Migrator}, prelude::FromRow, postgres::{PgQueryResult, PgRow, PgValueRef}, types::{Json, JsonRawValue}};

use crate::{config::config::Config, db::{db::DB, prompt::service::PromptServices, transport::transport_component::TransportComponent, types::{component_type_attributes, component_type_value::ComponentTypeValue, service::ComponentTypeService}}, error::{self, error::AppError, json::JsonError}};




pub const ELEMENTS: [&str;6] = ["name","size","value","info","manufacturer","label"];


// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Component{
//     pub id: Option<i32>,
//     pub name: String,
//     pub size: Option<String>,
//     pub value: Option<String>,
//     pub info: Option<String>,
//     pub stock: i32,
//     pub origin: Option<String>,
//     pub label: Option<String>,
//     pub image: Option<Vec<u8>>,
//     pub datasheet: Option<Vec<u8>>
// }

// #[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
// pub struct Component{
//     pub id: Option<i32>,
//     pub name: String,
//     pub size: Option<String>,
//     pub value: Option<String>,
//     pub info: Option<String>,
//     pub stock: i32,
//     pub origin: Option<String>,
//     pub label: Option<String>,
//     pub image: bool,
//     pub datasheet: bool
// }

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Component{
    pub id: Option<i32>,
    pub name: String,
    pub stock: i32,
    pub price: Option<f32>,
    pub manufacturer: Option<String>,
    pub label: Option<String>,
    pub image: bool,
    pub datasheet: bool,

}



impl Component{

    pub fn fmt(&self) -> String {

        format!(
            "id: {}\n name: {}\n stock: {}\n image: {}\n datasheet: {}",
            self.id.unwrap_or_else(|| 0),
            self.name.clone(),
            self.stock,
            self.image,
            self.datasheet
        )

        //return self.name.clone() + &self.size.clone().unwrap_or_else(|| {"none".to_string()}).clone();
    }

    pub fn to_vec(&self) -> Vec<Option<&str>> {

        vec![
            Some(self.name.as_str()),
            // self.size.as_deref(),
            // self.value.as_deref(),
            // self.info.as_deref(),
            self.manufacturer.as_deref(),
            self.label.as_deref(),
        ]

    }


}



// pub struct TransportComponent{
//     pub id: Option<i32>,
//     pub name: String,
//     pub size: Option<String>,
//     pub value: Option<String>,
//     pub info: Option<String>,
//     pub stock: i32,
//     pub origin: Option<String>,
//     pub label: Option<String>,
//     pub image: Option<Vec<u8>>,
//     pub datasheet: Option<Vec<u8>>
// }

// impl TransportComponent {
//     pub fn into(&self) -> Component {
//         Component { 
//             id: self.id.clone(),
//             name: self.name.clone(), 
//             size: self.size.clone(), 
//             value: self.value.clone(), 
//             info: self.info.clone(), 
//             stock: self.stock, 
//             origin: self.origin.clone(), 
//             label: self.label.clone(), 
//             image: self.image.is_some(), 
//             datasheet: self.datasheet.is_some()
//         }
//     }



//     pub fn 


// }