use std::todo;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{db::{class_instance::{class_instance::ClassInstance, transport_class_instance::TransportClassInstance}, db::DB}, error::error::AppError};



pub trait ClassInstanceServices {

    async fn add_transport_class_instance(&self, transport_class_instance: TransportClassInstance) -> Result<Uuid, AppError>;
    async fn remove_class_instance(&self, class_instance_id: Uuid) -> Result<(), AppError>;
    async fn get_class_instance(&self, class_instance_id: Uuid) -> Result<ClassInstance, AppError>;

    async fn get_class_instance_descendants(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn get_class_instance_ancestors(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;

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

        Ok(())

    }

    async fn get_class_instance(&self, class_instance_id: Uuid) -> Result<ClassInstance, AppError> {
        
        let result: ClassInstance = sqlx::query_as("SELECT * FROM class_instance WHERE class_instance_id = ($1)")
            .bind(class_instance_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)

    }


// "WITH RECURSIVE descendants AS (
	
// 	SELECT
// 		class_instance_id,
//       class_id,
//       parent,
//       0 AS depth
//    FROM class_instance
//    WHERE class_instance_id = '019f93f3-a2c2-7783-95b2-1af8c151aaa3'
   
//    UNION ALL
   
//    SELECT
//    	ci.class_instance_id,
//    	ci.class_id,
//    	ci.parent,
//    	d.depth + 1
//    FROM class_instance ci
//    JOIN descendants d
//    	ON ci.parent = d.class_instance_id
		
// )
// SELECT *
// FROM descendants
// ORDER BY DEPTH, class_instance_id;"

    /// only 1 level!
    async fn get_class_instance_descendants(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError> {

        let result: Vec<ClassInstance> = sqlx::query_as("
            SELECT * 
            FROM class_instance
            WHERE parent = (?)
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;

        
        // let result: ClassInstance = sqlx::query_as("
        //     WITH RECURSIVE tree AS (
        //         SELECT *, ARRAY[] AS ancestors
        //         FROM test WHERE parent_id IS NULL

        //         UNION ALL

        //         SELECT test.id, tree.ancestors || test.parent_id
        //         FROM test, tree
        //         WHERE test.parent_id = tree.id
        //     ) SELECT * FROM tree WHERE 3 = ANY(tree.ancestors);
        // ")

        todo!()
    }
    
    /// All the way to the root
    async fn get_class_instance_ancestors(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError> {
        
        let result: Vec<ClassInstance> = sqlx::query_as("

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
            SELECT 
                class_instance_id,
                class_id,
                parent
            FROM ancestors
            ORDER BY DEPTH;

        
        
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)
    }

}