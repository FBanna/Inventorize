use std::fmt::Display;

use sqlx::prelude::FromRow;

use crate::{db::types::component_type_attributes::ComponentTypeAttributes, error::{error::AppError, types::TypeError}};




#[derive(FromRow, Debug)]
pub struct ComponentType {
    pub id: i64,
    pub name: String,
    pub inherits: Option<i64>,
    pub attributes: Option<ComponentTypeAttributes>
}


impl Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{} - {}:\nattributes: {:#}\n\nschema: {:#}\n\nprompts: {:#}", self.id, self.name, self.attributes, self.schema, self.prompts)
        return write!(f, "{} - {} inherits {} \nAttributes: {}", self.id, self.name, self.inherits.map_or("none".to_owned(), |v| v.to_string()), {
            if let Some(v) = &self.attributes {
                return v.fmt(f)
            }

            return "NO ATTRIBUTES"
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