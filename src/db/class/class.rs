use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde_json::Value as Json;

use crate::error::{error::AppError, json::JsonError};


#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Class {
    pub class_id: Uuid,
    pub name: String,
    pub fields: Json,
    pub schema: Json
}

impl Class {

    pub fn verify_component_attributes(&self, attributes: &Json) -> Result<(), AppError> {

        let validator = jsonschema::validator_for(&self.schema).expect("ERROR: Could not make validator");

        let evaluation = validator.evaluate(attributes);

        match evaluation.flag().valid{
            true => return Ok(()),
            false => {

                let errors = evaluation.iter_errors().map(|err| -> String {

                    return err.error.to_string();

                }).fold("".to_string(), |acc, x| {
                    return format!("{}\n{}", acc, x);
                });


                Err(JsonError::ComponentTypeAttributesMalformed(errors).into())

            },
        }
    }
}