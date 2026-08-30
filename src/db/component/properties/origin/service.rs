use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{db::{component::properties::origin::component_origin::ComponentOrigin, db::DB}, error::error::AppError};





pub trait ComponentOriginServices {


    async fn add_component_origin(&self, component_origin: ComponentOrigin) -> Result<(), AppError>;

    async fn add_component_origin_with_executer(&self, component_origin: ComponentOrigin, executor: impl PgExecutor<'_>) -> Result<(), AppError>;

    async fn get_all_component_origin_for_component(&self, component_id: Uuid) -> Result<Vec<ComponentOrigin>, AppError>;

}

impl ComponentOriginServices for DB {

    async fn add_component_origin(&self, component_origin: ComponentOrigin) -> Result<(), AppError> {
        
        let _output = sqlx::query("INSERT INTO component_origin (origin_id, component_id, part_number, price) VALUES ($1, $2, $3, $4)")
            .bind(&component_origin.origin_id)
            .bind(&component_origin.component_id)
            .bind(&component_origin.part_number)
            .bind(&component_origin.price)
            .execute(&*self.pool)
            .await?;

        Ok(())

    }

    async fn add_component_origin_with_executer(&self, component_origin: ComponentOrigin, executor: impl PgExecutor<'_>) -> Result<(), AppError> {
        
        let _output = sqlx::query("INSERT INTO component_origin (origin_id, component_id, part_number, price) VALUES ($1, $2, $3, $4)")
            .bind(&component_origin.origin_id)
            .bind(&component_origin.component_id)
            .bind(&component_origin.part_number)
            .bind(&component_origin.price)
            .execute(executor)
            .await?;

        Ok(())

    }
    
    async fn get_all_component_origin_for_component(&self, component_id: Uuid) -> Result<Vec<ComponentOrigin>, AppError> {
        
        let component_origins: Vec<ComponentOrigin> = sqlx::query_as("SELECT * FROM component_origin WHERE component_id = ($1)")
            .bind(component_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(component_origins)

    }
    
    

}