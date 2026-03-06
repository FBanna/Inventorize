
use serde::{Serialize, Serializer, ser::SerializeStruct};
use serde_json::Value as JsonValue;
use sqlx::prelude::FromRow;



#[derive(FromRow, Debug)]
pub struct ComponentType {
    pub id: i32,
    pub name: String,
    pub attributes: JsonValue,
    pub schema: JsonValue,
    pub prompts: JsonValue
}

impl Serialize for ComponentType {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        

        let mut state: <S as Serializer>::SerializeStruct = serializer.serialize_struct("ComponentType",4)?;
        
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("attributes", &self.attributes)?;
        state.serialize_field("schema", &self.schema)?;
        state.serialize_field("prompts", &self.prompts)?;
        
        state.end()
    }

}

