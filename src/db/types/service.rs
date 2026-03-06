use sqlx::sqlite::SqliteQueryResult;

use crate::{db::{db::DB, types::transport_type::TransportComponentType}, error::error::AppError};



pub trait ComponentTypeService {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError>;
}


impl ComponentTypeService for DB {
    async fn add_type(&self, tc: &TransportComponentType) -> Result<SqliteQueryResult, AppError> {
        todo!()
    }
}