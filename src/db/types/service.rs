use serde_json::json;
use sqlx::sqlite::SqliteQueryResult;

use crate::{db::{db::DB, types::{component_type::ComponentType, transport_type::TransportComponentType}}, error::error::AppError};



pub trait ComponentTypeService {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError>;
    async fn remove_type(&self, id: i32) -> Result<SqliteQueryResult, AppError>;
    async fn get_type(&self, id: i32) -> Result<ComponentType, AppError>;
    async fn list_types(&self) -> Result<Vec<ComponentType>, AppError>;
    
}


impl ComponentTypeService for DB {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError> {

        let (schema, prompts) = tc.gen_schema_and_prompts()?;

        let result: SqliteQueryResult = sqlx::query("INSERT INTO types (name, attributes, schema, prompts) VALUES (?,?,?,?)")
            .bind(&tc.name)
            .bind(&tc.attributes)
            .bind(&schema)
            .bind(&prompts)
            .execute(&*self.pool)
            .await?;
        

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