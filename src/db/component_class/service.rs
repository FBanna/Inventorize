use std::{any::Any, collections::HashMap, fmt::Display, todo};

use sqlx::{Execute, PgExecutor, Postgres, QueryBuilder, query};
use uuid::Uuid;
use serde_json::Value as Json;

use crate::{db::{class::service::ClassServices, class_instance::class_instance::ClassInstance, component::component::{Component, ComponentWithAttributes}, component_class::component_class::{ComponentClass, FacetSearch, PagedComponentSearch, PagedComponentSearchResult, SearchFacets, UnitComponentClassSearch}, db::DB}, error::{error::AppError::{self, JsonError}, json::JsonErrors::IncorrectFieldsFound}};




pub trait ComponentClassServices {

    async fn add_component_class(&self, component_class: ComponentClass, executor: impl PgExecutor<'_>) -> Result<(), AppError>;
    async fn add_list_component_class(&self, component_class_list: Vec<ComponentClass>) -> Result<(), AppError>;

    async fn get_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<ComponentClass, AppError>;
    async fn get_components_from_class_instance(&self, class_instance_id: Uuid) -> Result<Vec<Component>, AppError>;
    async fn get_class_instances_from_component(&self, component_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;
    async fn remove_component_class(&self, component_id: Uuid, class_instance_id: Uuid) -> Result<(), AppError>;

    async fn search_components_with_attributes_on_component_class(&self, search: PagedComponentSearch) -> Result<PagedComponentSearchResult, AppError>;
    async fn get_facets_from_search_on_component_class(&self, search: FacetSearch) -> Result<SearchFacets, AppError>;

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
    async fn search_components_with_attributes_on_component_class(&self, search: PagedComponentSearch) -> Result<PagedComponentSearchResult, AppError> {


        let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
"SELECT 
    c.component_id,
    c.class_instance_id,
    c.name,
    c.stock,
    m.name manufacturer,
    l.name label,    
    component_classes.attributes,
    EXISTS (
        SELECT 1
        FROM component_image cimg
        WHERE cimg.component_id = c.component_id
    ) AS image
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

LEFT JOIN manufacturer m
    ON m.manufacturer_id = c.manufacturer_id

LEFT JOIN label l
    ON l.label_id = c.label_id");


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


        build_select(search.units, &mut query);

        query.push("\nORDER BY c.component_id DESC OFFSET ");
        query.push_bind(search.state.page_pos * search.state.page_size);
        query.push(" LIMIT ");
        query.push_bind(search.state.page_size + 1);


        let mut result: Vec<ComponentWithAttributes> = query
            .build_query_as()
            .fetch_all(&*self.pool).await?;


        if result.len() as i32 != search.state.page_size + 1 {

            return Ok(
                PagedComponentSearchResult {
                    has_next: false,
                    results: result
                }
            )

        }

        assert!(result.len() as i32 == search.state.page_size + 1);

        result.pop();

        Ok(PagedComponentSearchResult { 
            results: result, 
            has_next: true
        })

        
    }



    async fn get_facets_from_search_on_component_class(&self, search: FacetSearch) -> Result<SearchFacets, AppError> {

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("
            WITH RECURSIVE ancestors AS (
                SELECT
                    class_instance_id,
                    class_id,
                    parent,
                    0 AS depth
                FROM class_instance
                WHERE class_instance_id IS NOT DISTINCT FROM ");
        
        query.push_bind(search.root);

        query.push("\nUNION ALL

                SELECT
                    ci.class_instance_id,
                    ci.class_id,
                    ci.parent,
                    a.depth + 1
                FROM class_instance ci
                JOIN ancestors a
                    ON ci.class_instance_id = a.parent
            ),

            components AS (
            
                SELECT
                    c.component_id
                FROM component c

                JOIN component_class cc_root
                    ON cc_root.component_id = c.component_id
                    AND cc_root.class_instance_id = ($1)");

        build_select(search.units, &mut query);

        query.push(
            "\n),
            facets AS (

                SELECT
                    a.class_instance_id,
                    a.depth,
                    cl.name,
                    f.key,
                    f.value,
                    COUNT(*) AS cnt
                FROM components mc
                
                
                    
                JOIN component_class cc
                    ON cc.component_id = mc.component_id

                JOIN ancestors a
                    ON a.class_instance_id = cc.class_instance_id

                JOIN class cl
                    ON cl.class_id = a.class_id

                CROSS JOIN LATERAL jsonb_each(cc.attributes) AS f(key, value)

                GROUP BY
                    a.class_instance_id,
                    a.depth,
                    cl.name,
                    f.key,
                    f.value
            ),

            facet_values AS (
                SELECT
                    class_instance_id,
                    depth,
                    name,
                    KEY, 
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
                    DEPTH,
                    name,
                    KEY,
                    name
            ),
            class_facets AS (
                SELECT
                    class_instance_id,
                    DEPTH,
                    name,
                    jsonb_object_agg(key, values_json) AS facets
                FROM facet_values
                GROUP BY class_instance_id, DEPTH, NAME
            )
            SELECT jsonb_agg(
                jsonb_build_object(
                    'class_instance_id', class_instance_id,
                    'name', name,
                    'facets', facets
                )
                ORDER BY DEPTH, class_instance_id
            )
            FROM class_facets;"
        );

        // let mut query: QueryBuilder<Postgres> = QueryBuilder::new("
        //     WITH RECURSIVE ancestors AS (
        //         SELECT
        //             class_instance_id,
        //             class_id,
        //             parent,
        //             0 AS depth
        //         FROM class_instance
        //         WHERE class_instance_id IS NOT DISTINCT FROM ");
        
        // query.push_bind(search.root);

        // query.push("\nUNION ALL

        //         SELECT
        //             ci.class_instance_id,
        //             ci.class_id,
        //             ci.parent,
        //             a.depth + 1
        //         FROM class_instance ci
        //         JOIN ancestors a
        //             ON ci.class_instance_id = a.parent
        //     ),
                    
        //     facets AS (
        //         SELECT
        //             a.class_instance_id,
        //             a.depth,
        //             cl.name,
        //             f.key,
        //             f.value,
        //             COUNT(*) AS cnt
        //         FROM ancestors a 
                
        //         JOIN class cl
        //             ON cl.class_id = a.class_id
                    
        //         JOIN component_class cc
        //             ON cc.class_instance_id = a.class_instance_id

        //         CROSS JOIN LATERAL jsonb_each(cc.attributes) AS f(key, value)");


        // query.push("\nWHERE EXISTS (SELECT 1 FROM component_class ccsearch WHERE ccsearch.component_id = cc.component_id AND ccsearch.class_instance_id = ");
        
        // query.push_bind(search.root);

        // query.push(")");

        // // ADDS filtered facets BROKEN
        // //build_select_facets(search, &mut query);

        // query.push("\nGROUP BY
        //         a.class_instance_id, cl.name, f.key, f.value, a.depth
        
        //     ),
        //     facet_values AS (
        //         SELECT
        //             class_instance_id,
        //             depth,
        //             name,
        //             KEY, 
        //             jsonb_agg(
        //                 jsonb_build_object(
        //                     'value', value,
        //                     'count', cnt
        //                 )
        //                 ORDER BY cnt DESC
        //             ) AS values_json
        //         FROM facets
        //         GROUP BY
        //             class_instance_id,
        //             DEPTH,
        //             name,
        //             KEY,
        //             name
        //     ),
        //     class_facets AS (
        //         SELECT
        //             class_instance_id,
        //             DEPTH,
        //             name,
        //             jsonb_object_agg(key, values_json) AS facets
        //         FROM facet_values
        //         GROUP BY class_instance_id, DEPTH, NAME
        //     )
        //     SELECT jsonb_agg(
        //         jsonb_build_object(
        //             'class_instance_id', class_instance_id,
        //             'name', name,
        //             'facets', facets
        //         )
        //         ORDER BY DEPTH, class_instance_id
        //     )
        //     FROM class_facets;");


        println!("{}", query.sql().as_str());
        let result: SearchFacets = query.build_query_as().bind(search.root).fetch_one(&*self.pool).await?;

        Ok(result)
        

    }
        
    
}




// BUILD ACTUAL ATTRIBUTE SEARCHES
fn build_select(units: Vec<UnitComponentClassSearch>, q: &mut QueryBuilder<Postgres>) {


    let mut first = true;

    // build filtering for reach class instance
    for unit in &units {


        // check if they are all empty - THIS IS REALLY BAD. FIX THIS
        let mut is_empty = true;
        for (key, values) in unit.facets.clone() {
            if !values.is_empty() {
                is_empty = false;
            }
        }

        if is_empty {
            continue;
        }

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

        for (key, values) in &unit.facets {


            // CHECK IF EMPTY AND SKIP
            if values.is_empty() {
                continue;
            }

            q.push(" AND cc.attributes->");
            q.push_bind(key);

            q.push(" = ANY(");
            q.push_bind(values);
            q.push(")");

        }


        q.push(")");
        
    }


}



// BUILD ACTUAL ATTRIBUTE SEARCHES FOR FACETS
fn build_select_facets(search: &FacetSearch, q: &mut QueryBuilder<Postgres>) {


    let mut first = true;

    // build filtering for reach class instance
    for unit in &search.units {


        // check if they are all empty - THIS IS REALLY BAD. FIX THIS
        let mut is_empty = true;
        for (key, values) in unit.facets.clone() {
            if !values.is_empty() {
                is_empty = false;
            }
        }

        if is_empty {
            continue;
        }

        // MANAGE AND

        if first {
            q.push("\nWHERE ");
            first = false;
        } else {
            q.push("\nAND ");
        }


        // BUILD EXIST STATEMENT

        q.push("EXISTS (SELECT 1 FROM component_class cc WHERE cc.class_instance_id = a.class_instance_id AND cc.class_instance_id = ");
        
        q.push_bind(unit.class_instance_id);

        // BUILD SELECT STATEMENT

        for (key, values) in &unit.facets {


            // CHECK IF EMPTY AND SKIP
            if values.is_empty() {
                continue;
            }

            q.push(" AND cc.attributes->");
            q.push_bind(key);

            q.push(" = ANY(");
            q.push_bind(values);
            q.push(")");

        }


        q.push(")");
        
    }


}