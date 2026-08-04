use std::todo;

use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{db::{class::{class::Class, transport_class::TransportClass}, db::DB}, error::error::AppError};




pub trait ClassServices {

    async fn add_transport_class(&self, transport_class: TransportClass) -> Result<Uuid, AppError>;

    /// Must add / check for changes to fields!!
    //async fn update_class(&self, class: Class) -> Result<(), AppError>;
    async fn remove_class(&self, class_id: Uuid) -> Result<(), AppError>;
    async fn get_class(&self, class_id: Uuid) -> Result<Class, AppError>;
    async fn get_all_classes(&self) -> Result<Vec<Class>, AppError>;

    async fn get_class_from_class_instance(&self, class_instance_id: Uuid) -> Result<Class, AppError>;
    
}


impl ClassServices for DB {

    // Add class & verify attributes
    async fn add_transport_class(&self, transport_class: TransportClass) -> Result<Uuid, AppError> {
        
        let schema = transport_class.gen_schema_and_verify()?;

        let id: Uuid = sqlx::query_scalar("INSERT INTO class (name, fields, schema) VALUES ($1,$2, $3) RETURNING class_id")
            .bind(&transport_class.name)
            .bind(&transport_class.fields)
            .bind(schema)
            .fetch_one(&*self.pool)
            .await?;

        return Ok(id);
    }


    /// takes a type id and deletes it from class, class_instance and component_class
    async fn remove_class(&self, class_id: Uuid) -> Result<(), AppError> {
        
        sqlx::query("DELETE FROM class WHERE class_id = ($1)")
            .bind(class_id)
            .execute(&*self.pool)
            .await?;


        Ok(())
    }

    async fn get_class(&self, class_id: Uuid) -> Result<Class, AppError> {

        let result: Class = sqlx::query_as("SELECT * FROM class WHERE class_id = ($1)")
            .bind(class_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)
        
    }
    
    async fn get_all_classes(&self) -> Result<Vec<Class>, AppError> {

        let result: Vec<Class> = sqlx::query_as("SELECT * FROM class")
            .fetch_all(&*self.pool)
            .await?;

        Ok(result)

    }

    async fn get_class_from_class_instance(&self, class_instance_id: Uuid) -> Result<Class, AppError> {

        // SELECT
        //             ci.class_instance_id,
        //             ci.class_id,
        //             ci.parent,
        //             a.depth + 1
        //         FROM class_instance ci
        //         JOIN ancestors a
        //             ON ci.class_instance_id = a.parent

        let result: Class = sqlx::query_as("
            SELECT c.* FROM class c
                JOIN class_instance cl
                ON c.class_id = cl.class_id
                WHERE cl.class_instance_id = $1
        ")
        .bind(class_instance_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }
    
    

}