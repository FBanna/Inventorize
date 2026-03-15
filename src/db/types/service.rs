use std::fmt::format;

use serde_json::{Value as JsonValue, json};
use sqlx::{Execute, sqlite::SqliteQueryResult, types::JsonRawValue};

use crate::{db::{db::DB, types::{component_type::ComponentType, transport_type::TransportComponentType}}, error::{error::AppError, json::JsonError}};



pub trait ComponentTypeService {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError>;
    async fn remove_type(&self, id: i32) -> Result<SqliteQueryResult, AppError>;
    async fn get_type(&self, id: i32) -> Result<ComponentType, AppError>;
    async fn list_types(&self) -> Result<Vec<ComponentType>, AppError>;
    
}


impl ComponentTypeService for DB {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError> {

        let option = tc.gen_schema_and_prompts_and_attributes()?;

        let result: SqliteQueryResult = sqlx::query("INSERT INTO types (name, inherits) VALUES (?,?)")
            .bind(&tc.name)
            .bind(&tc.inherits)
            .execute(&*self.pool)
            .await?;


        if let Some((schema, prompts, attributes)) = option {

            let result: SqliteQueryResult = sqlx::query("INSERT INTO type_attribute (type_id, attributes, schema, prompts) VALUES (?,?,?,?)")
                .bind(result.last_insert_rowid())
                .bind(&attributes)
                .bind(schema)
                .bind(prompts)
                .execute(&*self.pool)
                .await?;



            // COLUMNS

            let array = attributes["attributes"].as_array().ok_or(JsonError::GenSchema)?;

            let starter: String = "(component_id INTEGER PRIMARY KEY".to_string();
            
            let columns: String = array.iter().try_fold(starter, |acc, attribute| make_columns(acc, attribute))?;

            let finished = columns + ",FOREIGN KEY(component_id) REFERENCES component(id)" + ")";
            println!("columns: {}", &finished);


            // SANITISE THIS STUFF

            let query = format!(
                "CREATE TABLE IF NOT EXISTS {}{}",
                "usertype_".to_owned() + &tc.name,
                finished

            );

            let result = sqlx::query(&query).execute(&*self.pool).await?;

        }
        

        
        

        Ok(result)
    }
    
    async fn remove_type(&self, id: i32) -> Result<SqliteQueryResult, AppError> {

        let result: SqliteQueryResult = sqlx::query("DELETE FROM types WHERE ROWID = (?)")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        // REMOVE ALL COMPONENTS OF TYPE (id)

        Ok(result)

    }

    async fn get_type(&self, id: i32) -> Result<ComponentType, AppError> {
        
        let result: ComponentType = sqlx::query_as("SELECT * FROM types WHERE ROWID = (?)")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)

    }
    
    async fn list_types(&self) -> Result<Vec<ComponentType>, AppError> {
        
        let result: Vec<ComponentType> = sqlx::query_as("SELECT * FROM types")
            .fetch_all(&*self.pool)
            .await?;

        Ok(result)

    }
    
    


}


fn make_columns(acc: String, attribute: &JsonValue) -> Result<String, AppError> {


    let name: String = attribute.get("name")
        .ok_or(JsonError::GenSchema)?
        .as_str()
        .ok_or(JsonError::GenSchema)?
        .to_owned();

    let object_type: String = attribute.get("object_type")
        .ok_or(JsonError::GenSchema)?
        .as_str()
        .ok_or(JsonError::GenSchema)?
        .to_owned();

    // (resistor TEXT,
    Ok(format!("{},{} {}", acc, name, object_type))
}