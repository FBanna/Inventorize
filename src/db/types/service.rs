use std::fmt::format;

use serde_json::{Value as JsonValue, json};
use sqlx::{Execute, any::AnyQueryResult, postgres::PgQueryResult, prelude::FromRow, types::JsonRawValue};

use crate::{db::{db::DB, types::{component_type::ComponentType, component_type_attributes::ComponentTypeAttributes, transport_type::{AttributeType, TransportComponentType}}}, error::{error::AppError, json::JsonError}};


#[derive(FromRow, Debug)]
struct Flat {
    pub id: i64,
    pub name: String,
    pub inherits: i32,
    pub fields: Option<JsonValue>,
    pub schema: Option<JsonValue>,
    pub prompts: Option<JsonValue>
}

pub trait ComponentTypeService {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<i64, AppError>;
    async fn remove_type(&self, id: i64) -> Result<PgQueryResult, AppError>;
    async fn get_type(&self, id: i64) -> Result<ComponentType, AppError>;
    async fn list_types(&self) -> Result<Vec<ComponentType>, AppError>;
    
}


impl ComponentTypeService for DB {

    /// takes a type and adds it along with any attributes it may have
    async fn add_type(&self, tc: &TransportComponentType) -> Result<i64, AppError> {


        println!("i am running this");

        let option = tc.gen_schema_and_prompts_and_attributes()?;

        println!("now I'm here");

        let id: i64 = sqlx::query_scalar("INSERT INTO type (name, inherits) VALUES ($1,$2) RETURNING type_id")
            .bind(&tc.name)
            .bind(&tc.inherits)
            .fetch_one(&*self.pool)
            .await?;

        println!("affected: {}", id);


        if let Some((schema, prompts, attributes)) = option {

            let result: PgQueryResult = sqlx::query("INSERT INTO type_attribute (type_id, fields, schema, prompts) VALUES ($1,$2,$3,$4)")
                .bind(id)
                .bind(&attributes)
                .bind(schema)
                .bind(prompts)
                .execute(&*self.pool)
                .await?;



            // // COLUMNS

            // let array = attributes["attributes"].as_array().ok_or(JsonError::GenSchema)?;

            // let starter: String = "(component_id INTEGER PRIMARY KEY".to_string();
            
            // let columns: String = array.iter().try_fold(starter, |acc, attribute| make_columns(acc, attribute))?;

            // let finished = columns + ",FOREIGN KEY(component_id) REFERENCES component(id)" + ")";
            // println!("columns: {}", &finished);

            // // SANITISE THIS STUFF

            // let query = format!(
            //     "CREATE TABLE IF NOT EXISTS {}{}",
            //     "usertype_".to_owned() + &tc.name,
            //     finished

            // );

            // let result = sqlx::query(&query).execute(&*self.pool).await?;

        }
        

        
        // TODO

        Ok(id)
    }
    

    /// takes a type id and deletes it from types, type_attributes and component_type
    async fn remove_type(&self, id: i64) -> Result<PgQueryResult, AppError> {

        let result: PgQueryResult = sqlx::query("DELETE FROM type WHERE type_id = ($1)")
            .bind(id)
            .execute(&*self.pool)
            .await?;

        // REMOVE ALL COMPONENTS OF TYPE (id)

        Ok(result)

    }

    async fn get_type(&self, id: i64) -> Result<ComponentType, AppError> {
        
        let r: Flat = sqlx::query_as("
        SELECT 
            t.type_id AS id,
            t.name,
            t.inherits,
            ta.fields,
            ta.schema,
            ta.prompts
        FROM type t
        LEFT JOIN type_attribute ta ON ta.type_id = t.type_id
        WHERE t.type_id = ($1)
        ")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;

        let output = ComponentType{
            id: r.id,
            name: r.name,
            inherits: r.inherits,
            attributes: match (r.fields, r.schema, r.prompts) {
                (Some(attributes), Some(schema), Some(prompts)) => {
                    Some(ComponentTypeAttributes {
                        attributes,
                        schema,
                        prompts,
                    })
                }
                _ => None,
            },
        };

        Ok(output)

    }
    
    async fn list_types(&self) -> Result<Vec<ComponentType>, AppError> {
        
        let r: Vec<Flat> = sqlx::query_as("
        
        SELECT 
            t.type_id as id,
            t.name,
            t.inherits,
            ta.attributes,
            ta.schema,
            ta.prompts
        FROM type t
        LEFT JOIN type_attribute ta ON ta.type_id = t.type_id
        
        ")
            .fetch_all(&*self.pool)
            .await?;

        let types: Vec<ComponentType> = r.iter().map(|t: &Flat| -> ComponentType {
            ComponentType{
                id: t.id,
                name: t.name.to_owned(),
                inherits: t.inherits,
                attributes: match (t.fields.to_owned(), t.schema.to_owned(), t.prompts.to_owned()) {
                    (Some(attributes), Some(schema), Some(prompts)) => {
                        Some(ComponentTypeAttributes {
                            attributes,
                            schema,
                            prompts,
                        })
                    }
                    _ => None,
                },
            }
        }).collect();

        Ok(types)
        
    }
    
    


}


// fn make_columns(acc: String, attribute: &JsonValue) -> Result<String, AppError> {


//     let name: String = attribute.get("name")
//         .ok_or(JsonError::GenSchema)?
//         .as_str()
//         .ok_or(JsonError::GenSchema)?
//         .to_owned();

//     // let object_type: String = attribute.get("object_type")
//     //     .ok_or(JsonError::GenSchema)?
//     //     .as_str()
//     //     .ok_or(JsonError::GenSchema)?
//     //     .to_owned();

//     let object_type = attribute.get("object_type")
//         .ok_or(JsonError::GenSchema)?
//         .to_owned();

//     let sql_type: AttributeType = serde_json::from_value(object_type)?;

//     // (resistor TEXT,
//     Ok(format!("{},{} {}", acc, name, sql_type.to_sql()))
// }