use uuid::Uuid;

use crate::{db::{class_instance::{class_instance::ClassInstance, transport_class_instance::TransportClassInstance}, db::DB}, error::error::AppError};



pub trait ClassInstanceServices {

    async fn add_transport_class_instance(&self, transport_class_instance: TransportClassInstance) -> Result<Uuid, AppError>;
    async fn remove_class_instance(&self, class_instance_id: Uuid) -> Result<(), AppError>;
    async fn get_class_instance(&self, class_instance_id: Uuid) -> Result<ClassInstance, AppError>;

    // async fn update_class_instance(&self, class_instance_id: Uuid, class_instance: ClassInstance) -> Result<Uuid, AppError>;

}


impl ClassInstanceServices for DB {

    async fn add_transport_class_instance(&self, transport_class_instance: TransportClassInstance) -> Result<Uuid, AppError> {
        
        let id: Uuid = sqlx::query_scalar("INSERT INTO class_instance (class_id, parent) VALUES ($1,$2) RETURNING class_instance_id")
            .bind(&transport_class_instance.class_id)
            .bind(&transport_class_instance.parent)
            .fetch_one(&*self.pool)
            .await?;

        return Ok(id);

    }

    async fn remove_class_instance(&self, class_instance_id: Uuid) -> Result<(), AppError> {
        
        sqlx::query("DELETE FROM class_instance WHERE class_instance_id = ($1)")
            .bind(class_instance_id)
            .execute(&*self.pool)
            .await?;

        // REMOVE ALL COMPONENTS OF TYPE (id) & inherited
        todo!();

        Ok(())

    }

    async fn get_class_instance(&self, class_instance_id: Uuid) -> Result<ClassInstance, AppError> {
        
        let result: ClassInstance = sqlx::query_as("SELECT * FROM class_instance WHERE class_instance_id = ($1)")
            .bind(class_instance_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)

    }

}