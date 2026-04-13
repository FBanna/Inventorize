
use std::fmt::Display;

use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::Value as JsonValue;
use sqlx::prelude::FromRow;

use crate::error::{error::AppError, json::JsonError};


#[derive(FromRow, Debug)]
pub struct ComponentTypeAttributes {
    pub attributes: JsonValue,
    pub schema: JsonValue,
    //pub prompts: JsonValue
}


impl Display for ComponentTypeAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "attributes: {:#}\n\nschema: {:#}", self.attributes, self.schema)
    }
}

// impl Serialize for ComponentType {

//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
        

//         let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("ComponentType",4)?;
        
//         state.serialize_field("id", &self.id)?;
//         state.serialize_field("name", &self.name)?;
//         state.serialize_field("attributes", &self.attributes)?;
//         state.serialize_field("schema", &self.schema)?;
//         state.serialize_field("prompts", &self.prompts)?;
        
//         state.end()
//     }

// }

impl ComponentTypeAttributes {

    pub fn veryify_attributes(&self, json: &JsonValue) -> Result<(), AppError> {


        println!("type: {}", self);

        let validator = jsonschema::validator_for(&self.schema).expect("ERROR: Could not make json validator");

        let evaluation = validator.evaluate(json);

        match evaluation.flag().valid {
            true => {
                println!("COMPONENT PASSED: {:#}", json); 
                return Ok(())
            },
            false => {

                
                let errors = evaluation.iter_errors().map(|err| -> String {

                    err.error.to_string()

                }).fold("".to_string(), |acc, x| {
                    format!("{}\n{}", acc, x)
                });


                Err(JsonError::ComponentAttributesMalformed(errors).into())




            }
        }

    }

}

