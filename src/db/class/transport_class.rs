use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as Json};

use crate::error::{error::AppError::{self, JsonError}, json::JsonErrors};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransportClass {
    pub name: String,
    pub fields: Json
}

#[derive(Debug)]
pub enum AttributeType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
}

impl AttributeType {

    pub fn to_json(&self) -> &str {
        match self {
            AttributeType::String => "string",
            AttributeType::Integer => "integer",
            AttributeType::Float => "number",
            AttributeType::Boolean => "boolean",
            AttributeType::Date => "date"
        }
    }

    pub fn from(from: &str) -> Result<Self, AppError> {
        match from {
            "string" => Ok(AttributeType::String),
            "integer" => Ok(AttributeType::Integer),
            "float" => Ok(AttributeType::Float),
            "boolean" => Ok(AttributeType::Boolean),
            "date" => Ok(AttributeType::Date),
            _ => Err(JsonError(JsonErrors::IncorrectFieldsFound))
        }
    } 

    pub fn to_html(&self) -> &str {
        match self {
            AttributeType::String => "text",
            AttributeType::Integer => "number",
            AttributeType::Float => "number",
            AttributeType::Boolean => "checkbox",
            AttributeType::Date => "date"
        }
    } 
}

impl TransportClass {


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
    fn verify_attributes(&self, attributes: &Json) -> Result<(), AppError> {

        let attribute_schema_str = include_str!("./attribute_schema.json");

        let schema: Json = serde_json::from_str(attribute_schema_str).unwrap();



        let validator = jsonschema::validator_for(&schema).expect("ERROR: Could not make json validator");

        validator.validate(attributes).map_err(|err| -> AppError {

            JsonError(JsonErrors::ClassAttributesMalformed(err.to_string()))
        })?;

        Ok(())       

    }

    /// Generates schema from verified attributes
    /// as a JsonValue eg.:
    /// 
    /// ```json
    ///     {
    ///     "type": "object",
    ///     "required": ["resistance", "package"],
    ///     "properties": {
    ///         
    ///         "resistance": { "type": "integer" },
    ///         "package": { "type": "string" }
    /// 
    ///     }
    /// }
    /// ```
    pub fn gen_schema_and_verify(&self) -> Result<Json, AppError> {


        self.verify_attributes(&self.fields)?;

        let mut map_schema: serde_json::Map<String,Json> = serde_json::Map::new();

        //let mut map_prompts: serde_json::Map<String,JsonValue> = serde_json::Map::new();



        let mut properties: serde_json::Map<String,Json> = serde_json::Map::new();
        let mut required: Vec<Json> = Vec::new();


        map_schema.insert("type".to_owned(), Json::String("object".to_owned()));


        //let array = self.fields["attributes"].as_array().ok_or(JsonErrors::GenSchema)?;
        let array = self.fields.as_array().ok_or(JsonErrors::GenSchema)?;

        // let required: Vec<String> = Vec::new();

        

        for attribute in array{

            let name = attribute.get("name")
                    .ok_or(JsonErrors::GenSchema)?
                    .as_str()
                    .ok_or(JsonErrors::GenSchema)?
                    .to_owned();

            // prompt_list.insert(
            //     name.clone(),
            //     JsonValue::Array(Vec::new())
            
            // );

            required.push(Json::String(name.clone()));


            properties.insert(

                name, 
                Json::Object({

                    let mut type_map = Map::new();

                    let object_type = attribute.get("object_type")
                        .ok_or(JsonErrors::GenSchema)?
                        .as_str()
                        .ok_or(JsonErrors::GenSchema)?
                        .to_owned();

                    let a_type: AttributeType = AttributeType::from(&object_type)?;

                    match a_type {
                        AttributeType::Date => {
                            type_map.insert(
                                "format".to_owned(),
                                Json::String(a_type.to_json().to_owned())
                            );

                            type_map.insert(
                                "type".to_owned(),
                                Json::String(AttributeType::String.to_json().to_owned())
                            );
                        },
                        _ => {
                            type_map.insert(
                                "type".to_owned(),
                                Json::String(a_type.to_json().to_owned())
                            );
                        }
                    }
                    

                    type_map


                })
            );
        }

        map_schema.insert("properties".to_owned(), Json::Object(properties));
        map_schema.insert("required".to_owned(), Json::Array(required));
        map_schema.insert("additionalProperties".to_owned(), Json::Bool(false));



        let schema = Json::Object(map_schema);

        // map_prompts.insert("prompts".to_owned(), JsonValue::Object(prompt_list));

        // let prompts = JsonValue::Object(map_prompts);



        println!("schema: {:#}", schema);

        Ok(schema)




    }

}