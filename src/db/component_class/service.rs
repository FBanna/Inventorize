use std::{any::Any, collections::HashMap, fmt::Display, todo};

use sqlx::{Execute, PgExecutor, Postgres, QueryBuilder};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::{db::{class::service::ClassServices, class_instance::class_instance::ClassInstance, component::component::{Component, ComponentWithAttributes}, component_class::component_class::{ComponentClass, ComponentSearch}, db::DB}, error::{error::AppError::{self, JsonError}, json::JsonErrors::IncorrectFieldsFound}};




pub trait ComponentClassServices {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError>;
    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError>;

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError>;
    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError>;
    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError>;

    async fn search_components_with_attributes_on_component_class(&self, search: ComponentSearch) -> Result<Vec<ComponentWithAttributes>, AppError>;

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
    

    /// takes in a json search query eg
    /// 
    /// ```no_run
    /// 
    ///     [
    ///         {
    ///             class_instance_id: UUID,
    ///             fields: {
    ///                 "resistance": [60, 120],
    ///                 "package": ["0402"]
    ///             }
    ///         },
    ///         {
    ///             ...
    ///         }
    /// 
    ///     ]
    /// 
    /// 
    /// ```
    /// 
    /// and returns a list of Component
    async fn search_components_with_attributes_on_component_class(&self, search: ComponentSearch) -> Result<Vec<ComponentWithAttributes>, AppError> {


        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
"SELECT 
    c.*,
    component_classes.attributes
FROM component c

CROSS JOIN LATERAL (
    SELECT jsonb_agg(
        jsonb_build_object(
            'class_instance_id', cc.class_instance_id,
            'attributes', cc.attributes
        )
    ) AS attributes
    FROM component_class cc
    WHERE cc.component_id = c.component_id
) component_classes

WHERE EXISTS (
    SELECT 1 FROM component_class cc
    WHERE cc.component_id = c.component_id 
    AND cc.class_instance_id = "
        );

        query.push_bind(search.root);
        query.push(")");

        for unit in search.units {


            // BUILD EXIST STATEMENT

            query.push("\nAND EXISTS (SELECT 1 FROM component_class cc WHERE cc.component_id = c.component_id AND cc.class_instance_id = ");
            
            query.push_bind(unit.class_instance_id);

            // BUILD SELECT STATEMENT

            for field in unit.facets {

                query.push(" AND cc.attributes->");
                query.push_bind(field.0);

                query.push(" = ANY(");
                query.push_bind(field.1);
                query.push(")");

            }


            query.push(")");


            
        }


        let result: Vec<ComponentWithAttributes> = query.build_query_as().fetch_all(&*self.pool).await?;


        Ok(result)

        
    }



// SELECT 
// 	c.*,
// 	attributes.component_classes
// FROM component C
// CROSS JOIN LATERAL (
//     SELECT jsonb_agg(
//         jsonb_build_object(
//             'class_instance_id', cc.class_instance_id,
//             'attributes', cc.attributes
//         )
//     ) AS component_classes
//     FROM component_class cc
//     WHERE cc.component_id = c.component_id
// ) attributes
// WHERE EXISTS (
// 	SELECT 1 FROM component_class cc
// 	WHERE cc.component_id = c.component_id 
// 	AND cc.class_instance_id = '019fb504-da13-750e-a676-255480b07fdc' -- root of search
// )


// AND EXISTS (
// 	SELECT 1 FROM component_class cc
// 	WHERE cc.component_id = c.component_id 
// 	AND cc.class_instance_id = '019fb504-da15-7f27-a167-be1231641349' 
// 	AND cc.attributes->'resistance' = ANY(ARRAY[TO_JSONB(60)]) 
// )




    
    
}


// // BUILD ACTUAL ATTRIBUTE SEARCHES
// fn build_select(search: HashMap<String, Vec<Json>>) -> String {


//     let mut query = QueryBuilder::default();

//     for field in search {

//         query.push(" AND cc.attributes->>");
//         query.push_bind(field.0);
//         query.push(" = ANY(");
//         query.push_bind(field.1);
//         query.push(")");

//     }

//     let result = query.build().sql().as_str().to_owned();

//     println!("select: {}", result);

//     return result;


// }