use std::{any::Any, collections::HashMap, fmt::Display, todo};

use sqlx::{Execute, PgExecutor, Postgres, QueryBuilder};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::{db::{class::service::ClassServices, class_instance::class_instance::ClassInstance, component::component::{Component, ComponentWithAttributes}, component_class::component_class::{ComponentClass, ComponentClassSearch}, db::DB}, error::{error::AppError::{self, JsonError}, json::JsonErrors::IncorrectFieldsFound}};




pub trait ComponentClassServices {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError>;
    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError>;

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError>;
    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError>;
    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError>;

    async fn search_components_on_component_class(&self, searches: Vec<ComponentClassSearch>) -> Result<Vec<ComponentWithAttributes>, AppError>;

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
    /// and returns a list of ComponentWithAttributes
    async fn search_components_on_component_class(&self, searches: Vec<ComponentClassSearch>) -> Result<Vec<ComponentWithAttributes>, AppError> {

        // let result: Vec<ComponentWithAttributes> = sqlx::query_as("
        //     SELECT
        //         cc.component_id,

        //         cl.name,
        //         cl.stock,
        //         cl.manufacturer,
        //         cl.label,

        //         cc.attributes
                
        //     FROM component_class cc
        //     JOIN component cl
        //         ON cl.component_id = cc.component_id

        //     WHERE cc.class_instance_id = ($1)
        //     AND attributes @> ($2)
        // ")
        // .bind(class_instance_id)
        // .bind(search)
        // .fetch_all(&*self.pool)
        // .await?;


        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM component c ");


        if (!searches.is_empty()) {
            query.push("WHERE \n");
        }


        let mut first: bool = true;

        for search in searches {


            // MANAGE 'AND EXISTS'
            if !first {

                query.push("AND ");
                
            } else {
                first = false;
            }

            // BUILD EXIST STATEMENT

            query.push("EXISTS (SELECT 1 FROM component_class cc WHERE cc.component_id = c.component_id AND cc.class_instance_id = ");
            
            query.push_bind(search.class_instance_id);

            // BUILD SELECT STATEMENT



            let class = self.get_class_from_class_instance(search.class_instance_id).await?;

            for potential_field in class.fields.as_array().ok_or(JsonError(IncorrectFieldsFound))? {

                let field_name = potential_field
                    .get("name")
                    .ok_or(JsonError(IncorrectFieldsFound))?
                    .to_string();

                if !search.facets.contains_key(&field_name) {
                    break;
                }

                

                query.push(" AND cc.attributes->>");
                query.push_bind(field.0);
                query.push(" = ANY(");

                


                query.push_bind(field.1);
                query.push(")");

            }

            for field in search.facets {

                // for value in field.1 {
                    
                //     class.fields.map

                // }


                query.push(" AND cc.attributes->>");
                query.push_bind(field.0);
                query.push(" = ANY(");


                query.push_bind(field.1);
                query.push(")");

            }


            query.push(")");


            
        }

        //query.push(";");
        
        // let built: sqlx::query::QueryAs<'_, Postgres, Vec<ComponentWithAttributes>, sqlx::postgres::PgArguments> = query.build_query_as();

        println!("query: \n\n{}", query.build().sql().as_str());


        let result: Vec<ComponentWithAttributes> = query.build_query_as().fetch_all(&*self.pool).await?;


        //query.build().sql().into()

        //Ok(result)

        Ok(result)
    }
    
    
}


// BUILD ACTUAL ATTRIBUTE SEARCHES
fn build_select(search: HashMap<String, Vec<Json>>) -> String {


    let mut query = QueryBuilder::default();

    for field in search {

        query.push(" AND cc.attributes->>");
        query.push_bind(field.0);
        query.push(" = ANY(");
        query.push_bind(field.1);
        query.push(")");

    }

    let result = query.build().sql().as_str().to_owned();

    println!("select: {}", result);

    return result;


}
// SELECT c.*
// FROM component c
// WHERE
//     EXISTS (
//         SELECT 1
//         FROM component_class cc
//         WHERE cc.component_id = c.component_id
//           AND cc.class_instance_id = $1
//           AND cc.attributes @> $2
//     )
// AND EXISTS (
//         SELECT 1
//         FROM component_class cc
//         WHERE cc.component_id = c.component_id
//           AND cc.class_instance_id = $3
//           AND cc.attributes @> $4
//     );


// EXISTS (
//     SELECT 1
//     FROM component_class cc
//     WHERE cc.component_id = c.component_id
//       AND cc.class_instance_id = $1
//       AND cc.attributes->>'package' = '0402'
//       AND cc.attributes->>'resistance' IN ('60','120')
// )