
use serde_json::{Map, Value as JsonValue, json};

use crate::error::{error::AppError, json::JsonError};

pub struct TransportComponentType {
    pub name: String,
    pub inherits: i32,
    pub attributes: Option<JsonValue>
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
    fn verify_attributes(&self, attributes: &JsonValue) -> Result<(), AppError> {


        let attribute_schema_str = include_str!("attribute_schema.json");

        let schema: JsonValue = serde_json::from_str(attribute_schema_str).unwrap();


        //println!("schema: {:#}\n\n", schema);
        //println!("{:#}", self.attributes);

        let validator = jsonschema::validator_for(&schema).expect("ERROR: Could not make json validator");

        let evaluation = validator.evaluate(attributes);

        match evaluation.flag().valid{
            true => return Ok(()),
            false => {

                let errors = evaluation.iter_errors().map(|err| -> String {

                    err.error.to_string()

                }).fold("".to_string(), |acc, x| {
                    format!("{}\n{}", acc, x)
                });


                Err(JsonError::ComponentTypeAttributesMalformed(errors).into())


            },
        }

        

    }

    /// Generates schema from verified attributes returning it
    /// as a JsonValue eg.:
    /// 
    /// ```json
    ///     {
    ///     "type": "object",
    ///     "properties": {
    ///         
    ///         "resistance": { "type": "integer" },
    ///         "package": { "type": "string" }
    /// 
    ///     }
    /// }
    /// ```
    pub fn gen_schema_and_prompts_and_attributes(&self) -> Result<Option<(JsonValue, JsonValue, JsonValue)>, AppError> {

        if self.attributes.is_none(){
            return Ok(None);
        }

        let attributes = self.attributes.clone().unwrap();

        self.verify_attributes(&attributes)?;

        let mut map_schema: serde_json::Map<String,JsonValue> = serde_json::Map::new();

        let mut map_prompts: serde_json::Map<String,JsonValue> = serde_json::Map::new();



        let mut properties: serde_json::Map<String,JsonValue> = serde_json::Map::new();

        let mut prompt_list: serde_json::Map<String,JsonValue> = serde_json::Map::new();

        map_schema.insert("type".to_owned(), JsonValue::String("object".to_owned()));

        let array = attributes["attributes"].as_array().ok_or(JsonError::GenSchema)?;

        // let required: Vec<String> = Vec::new();

        

        for attribute in array{

            let name = attribute.get("name")
                    .ok_or(JsonError::GenSchema)?
                    .as_str()
                    .ok_or(JsonError::GenSchema)?
                    .to_owned();

            prompt_list.insert(
                name.clone(),
                JsonValue::Array(Vec::new())
            
            );


            properties.insert(

                name, 
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

        map_schema.insert("properties".to_owned(), JsonValue::Object(properties));

        let schema = JsonValue::Object(map_schema);

        map_prompts.insert("prompts".to_owned(), JsonValue::Object(prompt_list));

        let prompts = JsonValue::Object(map_prompts);



        println!("schema: {:#}\nprompts: {:#}", schema, prompts);

        Ok(Some((schema, prompts, attributes)))




    }

}