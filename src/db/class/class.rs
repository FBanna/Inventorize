use std::println;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde_json::Value as Json;

use crate::error::{error::AppError::{self, JsonError}, json::JsonErrors};



#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Class {
    pub class_id: Uuid,
    pub name: String,
    pub fields: Json,
    pub schema: Json
}

impl Class {

    pub fn verify_component_attributes(&self, attributes: &Json) -> Result<(), AppError> {

        let validator = jsonschema::validator_for(&self.schema)?;

        validator.validate(attributes).map_err(|err| -> AppError {

            return JsonError(JsonErrors::ComponentClassAttributesMalformed(err.to_string()));

        })?;

        Ok(())

    }
}