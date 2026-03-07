use serde_json::json;
use sqlx::sqlite::SqliteQueryResult;

use crate::{db::{db::DB, types::transport_type::TransportComponentType}, error::error::AppError};



pub trait ComponentTypeService {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError>;
}


impl ComponentTypeService for DB {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError> {

        let schema = tc.gen_schema()?;

        let result: SqliteQueryResult = sqlx::query("INSERT INTO types (name, attributes, schema, prompts) VALUES (?,?,?,?)")
            .bind(&tc.name)
            .bind(&tc.attributes)
            .bind(&schema)
            .bind(json!({"help": "please"}))
            .execute(&*self.pool)
            .await?;
        

        Ok(result)
    }
}