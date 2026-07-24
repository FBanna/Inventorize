use std::todo;

use uuid::Uuid;

use crate::{db::{class::{class::Class, transport_class::TransportClass}, db::DB}, error::error::AppError};




pub trait ClassServices {

    async fn add_transport_class(&self, transport_class: TransportClass) -> Result<Uuid, AppError>;

    /// Must add / check for changes to fields!!
    //async fn update_class(&self, class: Class) -> Result<(), AppError>;
    async fn remove_class(&self, class_id: Uuid) -> Result<(), AppError>;
    async fn get_class(&self, class_id: Uuid) -> Result<Class, AppError>;
    async fn get_class_ancestors_from_instance(&self, class_instance_id: Uuid) -> Result<Vec<Class>, AppError>;
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

        // REMOVE ALL COMPONENTS OF TYPE (id) & inherited
        todo!();

        Ok(())
    }

    async fn get_class(&self, class_id: Uuid) -> Result<Class, AppError> {

        let result: Class = sqlx::query_as("SELECT * FROM class WHERE class_id = ($1)")
            .bind(class_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)
        
    }
    
    async fn get_class_ancestors_from_instance(&self, class_instance_id: Uuid) -> Result<Vec<Class>, AppError> {
        
        let result: Vec<Class> = sqlx::query_as("

            WITH RECURSIVE ancestors AS (
                -- Base case: start with the requested node
                SELECT
                    class_instance_id,
                    class_id,
                    parent,
                    0 AS depth
                FROM class_instance
                WHERE class_instance_id = '019f93f3-a2c4-7b5a-989a-2ca8e6610ae9'

                UNION ALL

                -- Recursive step: find the parent of the current row
                SELECT
                    ci.class_instance_id,
                    ci.class_id,
                    ci.parent,
                    a.depth + 1
                FROM class_instance ci
                JOIN ancestors a
                    ON ci.class_instance_id = a.parent
            )
            SELECT c.*
            FROM ancestors a
            JOIN class c
                ON c.class_id = a.class_id
            ORDER BY a.depth;      
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)

    }

}