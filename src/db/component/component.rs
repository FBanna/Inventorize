use std::{collections::HashMap, fs, io::Cursor, path::{Path, PathBuf}};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Execute, Pool, QueryBuilder, Row, Postgres, PgPool, migrate::{MigrateDatabase, Migrator}, prelude::FromRow, postgres::{PgQueryResult, PgRow, PgValueRef}};
use uuid::Uuid;
use serde_json::Value as Json;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Component{
    pub component_id: Uuid,
    pub class_instance_id: Uuid,
    
    pub name: String,
    pub stock: i32, // should this be i32?
    pub manufacturer: Option<String>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentWithAttributes{
    pub component_id: Uuid,
    pub class_instance_id: Uuid,

    pub name: String,
    pub stock: i32, // should this be i32?
    pub manufacturer: Option<String>,
    pub label: Option<String>,

    pub attributes: Json
}





impl Component{

    pub fn fmt(&self) -> String {

        return format!(
            "id: {}\nname: {}\nstock: {}\nmanufacturer: {}\nlabel: {}",
            self.component_id,
            self.name,
            self.stock,
            self.manufacturer.clone().unwrap_or_default(),
            self.label.clone().unwrap_or_default()
        );
    }

}


impl ComponentWithAttributes{

    pub fn fmt(&self) -> String {

        return format!(
            "id: {}\nname: {}\nstock: {}\nmanufacturer: {}\nlabel: {}\nattributes: {:#?}",
            self.component_id,
            self.name,
            self.stock,
            self.manufacturer.clone().unwrap_or_default(),
            self.label.clone().unwrap_or_default(),
            self.attributes
        );
    }

}