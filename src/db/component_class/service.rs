use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{db::{class_instance::class_instance::ClassInstance, component::component::Component, component_class::component_class::ComponentClass, db::DB}, error::error::AppError};




pub trait ComponentClassServices {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError>;
    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError>;

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError>;
    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError>;
    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError>;

    async fn update_component_class(&self, component_class: ComponentClass) -> Result<(), AppError>;

}

impl ComponentClassServices for DB {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError> {
        
        sqlx::query("INSERT INTO component_class (component_id, class_instance_id, attributes) VALUES ($1,$2,$3)")
            .bind(component_class.component_id)
            .bind(component_class.class_instance_id)
            .bind(component_class.attributes)
            .execute(executor)
            .await?;

        return Ok(());

    }

    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        for component_class in component_class_list {

            sqlx::query("INSERT INTO component_class (component_id, class_instance_id, attributes) VALUES ($1,$2,$3)")
                .bind(component_class.component_id)
                .bind(component_class.class_instance_id)
                .bind(component_class.attributes)
                .execute(&mut *tx)
                .await?;

        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError> {
        
        let result: ComponentClass = sqlx::query_as("
            SELECT * FROM component_class 
            WHERE component_id = ($1)
            AND class_instance_id = ($2)
        ")
        .bind(component_id)
        .bind(class_instance_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError> {
        let result: Vec<Component> = sqlx::query_as("
            SELECT * FROM component_class 
            WHERE class_instance_id = ($1)
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError> {
        let result: Vec<ClassInstance> = sqlx::query_as("
            SELECT * FROM component_class 
            WHERE component_id = ($1)
        ")
        .bind(component_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError> {
        
        sqlx::query("
            DELETE FROM component_class
            WHERE component_id = ($1)
            AND class_instance_id = ($2)
        ")
        .bind(component_id)
        .bind(class_instance_id)
        .execute(&*self.pool).await?;

        Ok(())

    }

    async fn update_component_class(&self, component_class: ComponentClass) -> Result<(), AppError> {
        
        sqlx::query("
            UPDATE component_class
            SET
                attributes = ($1)
            WHERE component_id = ($2)
            AND class_instance_id = ($3)
        ")
        .bind(component_class.attributes)
        .bind(component_class.component_id)
        .bind(component_class.class_instance_id)
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
    
    
}