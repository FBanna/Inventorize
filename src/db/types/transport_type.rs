
use serde_json::{Map, Value as JsonValue, json};

use crate::error::{error::AppError, json::JsonError};

pub struct TransportComponentType {
    pub name: String,
    pub attributes: JsonValue
}

impl TransportComponentType {


    /// Takes in rough attributes from frontend and validates it
    /// 
    /// 
    /// eg. {
    ///     "resistance": { "type": "integer", "unit": "ohms" },
    ///     "package": { "type": "string"}
    /// 
    /// }
    /// 
    /// 
    fn verify_attributes(&self) -> Result<(), AppError> {

        let attribute_schema_str = include_str!("attribute_schema.json");

        let schema: JsonValue = serde_json::from_str(attribute_schema_str).unwrap();


        println!("schema: {:#}\n\n", schema);
        println!("{:#}", self.attributes);

        let validator = jsonschema::validator_for(&schema).expect("ERROR: Could not make json validator");

        let evaluation = validator.evaluate(&self.attributes);

        match evaluation.flag().valid{
            true => return Ok(()),
            false => {

                let errors = evaluation.iter_errors().map(|err| -> String {

                    err.error.to_string()

                }).fold("".to_string(), |acc, x| {
                    format!("{}\n{}", acc, x)
                });


                Err(JsonError::TypeAttributeEvaluation(errors).into())


            },
        }

        

    }

    /// Generates schema from verified attributes returning it
    /// as a JsonValue
    /// 
    /// {
    ///     "type": "object":
    ///     "properties": {
    ///         
    ///         "resistance": { "type": "integer" },
    ///         "package": { "type": "string" }
    /// 
    ///     }
    /// }
    pub fn gen_schema(&self) -> Result<JsonValue, AppError> {

        self.verify_attributes()?;

        let mut map: serde_json::Map<String,JsonValue> = serde_json::Map::new();

        let mut properties: serde_json::Map<String,JsonValue> = serde_json::Map::new();

        map.insert("type".to_owned(), JsonValue::String("object".to_owned()));

        let array = self.attributes["attributes"].as_array().ok_or(JsonError::GenSchema)?;

        // let required: Vec<String> = Vec::new();

        for attribute in array{
            properties.insert(

                attribute.get("name")
                    .ok_or(JsonError::GenSchema)?
                    .as_str()
                    .ok_or(JsonError::GenSchema)?
                    .to_owned(), 


                JsonValue::Object({

                    let mut type_map = Map::new();

                    type_map.insert(
                        "type".to_owned(),

                        attribute.get("object_type")
                            .ok_or(JsonError::GenSchema)?.to_owned()

                    );

                    type_map


                })
            );
        }

        map.insert("properties".to_owned(), JsonValue::Object(properties));

        let value = JsonValue::Object(map);

        println!("schema: {:#}", value);

        Ok(value)




    }

}