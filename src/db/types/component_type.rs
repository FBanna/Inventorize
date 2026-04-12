use std::fmt::Display;

use sqlx::prelude::FromRow;

use crate::{db::types::component_type_attributes::ComponentTypeAttributes, error::{error::AppError, types::TypeError}};




#[derive(FromRow, Debug)]
pub struct ComponentType {
    pub id: i64,
    pub name: String,
    pub inherits: i32,
    pub attributes: Option<ComponentTypeAttributes>
}


impl Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{} - {}:\nattributes: {:#}\n\nschema: {:#}\n\nprompts: {:#}", self.id, self.name, self.attributes, self.schema, self.prompts)
        write!(f, "{} - {} inherits {} \nAttributes: {}", self.id, self.name, self.inherits, {
            if let Some(v) = &self.attributes {
                return v.fmt(f)
            }

            "NO ATTRIBUTES"
        })
    
    }
}

impl ComponentType {
    pub fn get_attributes(&self) -> Result<&ComponentTypeAttributes, AppError> {
        match &self.attributes {
            Some(a) => Ok(a),
            None => Err(TypeError::ExpectedAttributes.into()),
        }
    }
}