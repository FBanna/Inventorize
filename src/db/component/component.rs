use std::{collections::HashMap, fs, io::Cursor, path::{Path, PathBuf}};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Execute, Pool, QueryBuilder, Row, Postgres, PgPool, migrate::{MigrateDatabase, Migrator}, prelude::FromRow, postgres::{PgQueryResult, PgRow, PgValueRef}};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::{db::{class::{service::ClassServices, transport_class::AttributeType}, class_instance::{class_instance::ClassClassInstance, service::ClassInstanceServices}, db::DB}, error::{error::AppError::{self, JsonError}, json::JsonErrors}};

pub static CORE_ATTRIBUTES: [&str; 4] = ["name", "stock", "manufacturer", "label"];

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Component{
    pub component_id: Uuid,
    pub class_instance_id: Uuid,
    
    pub name: String,
    pub stock: i32, // should this be i32?
    pub manufacturer_id: Option<Uuid>,
    pub label_id: Option<Uuid>,
}


/// Search Result Object -- should be as close to a finished object as possible ( - image)
#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct  ComponentWithAttributes{
    pub component_id: Uuid,
    pub class_instance_id: Uuid,

    pub name: String,
    pub stock: i32, // should this be i32?

    pub manufacturer: Option<String>, // string to display
    pub label: Option<String>,

    pub attributes: Json,

    pub origins: Json,

    pub image: bool
}





impl Component{

    pub fn fmt(&self) -> String {

        return format!(
            "id: {}\nname: {}\nstock: {}\nmanufacturer: {}\nlabel: {}",
            self.component_id,
            self.name,
            self.stock,
            self.manufacturer_id.unwrap_or_default().hyphenated().to_string(),
            self.label_id.unwrap_or_default().hyphenated().to_string()
        );
    }

}


impl ComponentWithAttributes{

    pub fn fmt(&self) -> String {

        return format!(
            "id: {}\nname: {}\nstock: {}\nmanufacturer: {:#?}\nlabel: {:#?}\nattributes: {:#?}",
            self.component_id,
            self.name,
            self.stock,
            self.manufacturer,
            self.label,
            self.attributes
        );
    }


    // pub async fn convert_number_to_prefix(&self, db: &DB) -> Result<Self, AppError> {

    //     let ancestors: Vec<ClassClassInstance> = db.get_class_ancestors_from_instance(Some(self.class_instance_id)).await?;


    //     for class_class in ancestors{
    //         for field in class_class.fields.as_array().ok_or(JsonError(JsonErrors::IncorrectFieldsFound))? {

    //             let object_type = field.get("object_type")
    //                 .ok_or(JsonErrors::GenSchema)?
    //                 .as_str()
    //                 .ok_or(JsonErrors::GenSchema)?;

    //             let attribute_type: AttributeType = AttributeType::from(object_type)?;

    //             match attribute_type {
    //                 AttributeType::Prefix => {
                        
    //                     // actually change value here
                        
    //                     self.attributes.get_mut(index)

    //                 },
    //                 _ 
    //             }

    //         }
    //     }
        

    //     todo!()


    // }

}