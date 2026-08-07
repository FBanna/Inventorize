use std::{any::Any, collections::HashMap, fmt::Display, todo};

use sqlx::{Execute, PgExecutor, Postgres, QueryBuilder};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::{db::{class::service::ClassServices, class_instance::class_instance::ClassInstance, component::component::{Component, ComponentWithAttributes}, component_class::component_class::{ComponentClass, ComponentSearch, SearchFacets}, db::DB}, error::{error::AppError::{self, JsonError}, json::JsonErrors::IncorrectFieldsFound}};




pub trait ComponentClassServices {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError>;
    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError>;

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError>;
    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError>;
    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError>;

    async fn search_components_with_attributes_on_component_class(&self, search: ComponentSearch) -> Result<Vec<ComponentWithAttributes>, AppError>;
    async fn get_facets_from_search_on_component_class(&self, search: ComponentSearch) -> Result<SearchFacets, AppError>;

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
) component_classes"
        );


        // Only select components of a certain type
        if let Some(root) = search.root {
            query.push(
            "\nJOIN component_class root
                ON root.component_id = c.component_id
                AND root.class_instance_id ="
            );

            query.push_bind(root);
            //query.push(")");
        }

        build_select(search, &mut query);

        // println!("{}", query.build().sql().as_str());


        let result: Vec<ComponentWithAttributes> = query.build_query_as().fetch_all(&*self.pool).await?;


        Ok(result)

        
    }
    
    async fn get_facets_from_search_on_component_class(&self, search: ComponentSearch) -> Result<SearchFacets, AppError> {
        
        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
            "
WITH facets AS (
    SELECT
        cc.class_instance_id,
        cl.name,
        f.key,
        f.value,
        COUNT(*) AS cnt
    FROM component c");

        // Only select components of a certain type
        if let Some(root) = search.root {
            query.push(
            "\nJOIN component_class root
                ON root.component_id = c.component_id
                AND root.class_instance_id ="
            );

            query.push_bind(root);
            //query.push(")");
        }

        query.push(
            "\nJOIN component_class cc
                ON cc.component_id = c.component_id

            JOIN class_instance ci
                ON ci.class_instance_id = cc.class_instance_id

            JOIN class cl
                ON cl.class_id = ci.class_id

            CROSS JOIN LATERAL jsonb_each(cc.attributes) AS f(key, value)");


        build_select(search, &mut query);

        query.push(
"GROUP BY
    cc.class_instance_id,
    cl.name,
    f.key,
    f.value
),

facet_values AS (
    SELECT
        class_instance_id,
        name,
        key,
        jsonb_agg(
            jsonb_build_object(
                'value', value,
                'count', cnt
            )
            ORDER BY cnt DESC
        ) AS values_json
    FROM facets
    GROUP BY
        class_instance_id,
        name,
        key
),

class_facets AS (
    SELECT
        class_instance_id,
        name,
        jsonb_object_agg(key, values_json) AS facets
    FROM facet_values
    GROUP BY class_instance_id, name
)

SELECT jsonb_agg(
    jsonb_build_object(
        'class_instance_id', class_instance_id,
        'name', name,
        'facets', facets
    )
    ORDER BY class_instance_id
)
FROM class_facets;");

        

        let result: SearchFacets = query.build_query_as().fetch_one(&*self.pool).await?;


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




// BUILD ACTUAL ATTRIBUTE SEARCHES
fn build_select(search: ComponentSearch, q: &mut QueryBuilder<Postgres>) {


    let mut first = true;

    // build filtering for reach class instance
    for unit in search.units {

        // MANAGE AND

        if first {
            q.push("\nWHERE ");
            first = false;
        } else {
            q.push("\nAND ");
        }


        // BUILD EXIST STATEMENT

        q.push("EXISTS (SELECT 1 FROM component_class cc WHERE cc.component_id = c.component_id AND cc.class_instance_id = ");
        
        q.push_bind(unit.class_instance_id);

        // BUILD SELECT STATEMENT

        for field in unit.facets {

            q.push(" AND cc.attributes->");
            q.push_bind(field.0);

            q.push(" = ANY(");
            q.push_bind(field.1);
            q.push(")");

        }


        q.push(")");
        
    }


}