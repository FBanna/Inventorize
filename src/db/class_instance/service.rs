use std::{println, todo};

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde_json::{Map, Value as Json};

use crate::{db::{class::{class::Class, transport_class::AttributeType}, class_instance::{class_instance::{ClassClassInstance, ClassInstance, ClassInstanceTree, ClassInstanceTreeLeaf}, transport_class_instance::TransportClassInstance}, component::component::CORE_ATTRIBUTES, db::DB}, error::{class::ClassErrors, error::AppError::{self, JsonError}, json::JsonErrors::ComponentClassAttributesMalformed}};




pub trait ClassInstanceServices {

    async fn add_transport_class_instance(&self, transport_class_instance: TransportClassInstance) -> Result<Uuid, AppError>;
    async fn remove_class_instance(&self, class_instance_id: Uuid) -> Result<(), AppError>;
    async fn get_class_instance(&self, class_instance_id: Uuid) -> Result<ClassInstance, AppError>;

    async fn get_class_instance_descendants(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassInstanceTree>, AppError>;
    async fn get_class_instance_descendants_child(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassInstance>, AppError>;
    async fn get_class_instance_ancestors(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError>;

    async fn get_class_ancestors_from_instance(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassClassInstance>, AppError>;


    async fn build_facet_list_for_instance(&self, class_instance_id: Option<Uuid>) -> Result<Json, AppError>;

    async fn build_facet_list_for_instance_for_html(&self, class_instance_id: Option<Uuid>) -> Result<Json, AppError>;

    //async fn get_facets_from_instance(&self, class_instance_id: Option<Uuid>) -> Result<>

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

    /// excludes the starting id
    async fn get_class_instance_descendants(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassInstanceTree>, AppError> {
        
        let result: Vec<ClassInstanceTreeLeaf> = sqlx::query_as("
        WITH RECURSIVE descendants AS (

                SELECT
                    class_instance_id,
                    class_id,
                    parent,
                    0 AS depth
                FROM class_instance
                WHERE parent IS NOT DISTINCT FROM ($1)
                
                UNION ALL
                
                SELECT
                    ci.class_instance_id,
                    ci.class_id,
                    ci.parent,
                    d.depth + 1
                FROM class_instance ci
                JOIN descendants d
                    ON ci.parent = d.class_instance_id
                        
            )
            SELECT 
                d.class_instance_id,
                c.name,
                d.parent
            FROM descendants d
            JOIN class c
                ON c.class_id = d.class_id
            ORDER BY d.depth, class_instance_id;
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;

        return Ok(ClassInstanceTree::to_tree(result));
    }




    /// only 1 level!
    async fn get_class_instance_descendants_child(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassInstance>, AppError> {

        let result: Vec<ClassInstance> = sqlx::query_as("
            SELECT * 
            FROM class_instance
            WHERE parent IS NOT DISTINCT FROM ($1)
        ")
        .bind(class_instance_id)
        .fetch_all(&*self.pool)
        .await?;


        Ok(result)
    }
    
    /// All the way to the root
    async fn get_class_instance_ancestors(&self, class_instance_id: Uuid) -> Result<Vec<ClassInstance>, AppError> {
        
        let result: Vec<ClassInstance> = sqlx::query_as("

            WITH RECURSIVE ancestors AS (
                SELECT
                    class_instance_id,
                    class_id,
                    parent,
                    0 AS depth
                FROM class_instance
                WHERE class_instance_id IS NOT DISTINCT FROM ($1)

                UNION ALL

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

    async fn get_class_ancestors_from_instance(&self, class_instance_id: Option<Uuid>) -> Result<Vec<ClassClassInstance>, AppError> {
        
        let result: Vec<ClassClassInstance> = sqlx::query_as("

            WITH RECURSIVE ancestors AS (
                SELECT
                    class_instance_id,
                    class_id,
                    parent,
                    0 AS depth
                FROM class_instance
                WHERE class_instance_id IS NOT DISTINCT FROM ($1)

                UNION ALL

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
                c.class_id,
                c.name,
                c.fields,
                c.schema,
                a.class_instance_id
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
    

    /// {
    ///     core: ["name", "stock", "manufacturer"],
    ///     attributes: [
    ///         {
    ///             uuid: .....,
    ///             fields: [....]
    ///         },
    /// 
    ///     ]
    /// }
    async fn build_facet_list_for_instance(&self, class_instance_id: Option<Uuid>) -> Result<Json, AppError> {
        

        let result = self.get_class_ancestors_from_instance(class_instance_id).await?;

        

        let mut json: Map<String, Json> = Map::new();

        json.insert("core".to_string(), Json::from(CORE_ATTRIBUTES));

        let mut attributes: Vec<Json> = Vec::new();

        for class in result {

            let mut json_attribute = Map::new();

            json_attribute.insert("class_instance_id".to_owned(), class.class_instance_id.hyphenated().to_string().into());

            json_attribute.insert("name".to_owned(), class.name.into());

            json_attribute.insert(
                "fields".to_owned(), 
                class.fields
                );

            attributes.push(
                Json::Object(json_attribute)
            );
        }

        json.insert("attributes".to_owned(), attributes.into());

        Ok(Json::Object(json))
    }


    
    async fn build_facet_list_for_instance_for_html(&self, class_instance_id: Option<Uuid>) -> Result<Json, AppError> {
        
        let result = self.get_class_ancestors_from_instance(class_instance_id).await?;

        

        let mut json: Map<String, Json> = Map::new();

        json.insert("core".to_string(), Json::from(CORE_ATTRIBUTES));

        let mut attributes: Vec<Json> = Vec::new();

        for class in result {

            let mut json_attribute = Map::new();

            json_attribute.insert("class_instance_id".to_owned(), class.class_instance_id.hyphenated().to_string().into());

            json_attribute.insert("name".to_owned(), class.name.into());

            // custom logic to change object_type

            let mut fields = Vec::new();

            for field in class.fields
                .as_array()
                .ok_or(JsonError(ComponentClassAttributesMalformed("Fields is not an array".to_owned())))? {

                    let object_type = field.get("object_type")
                        .ok_or(JsonError(ComponentClassAttributesMalformed("Could not find Object Type".to_owned())))?
                        .as_str()
                        .ok_or(JsonError(ComponentClassAttributesMalformed("Expected string".to_owned())))?;

                    let converted_type = AttributeType::from(object_type)?;

                    let mut out_field = field.as_object().ok_or(JsonError(ComponentClassAttributesMalformed("Could not find map".to_owned())))?.to_owned();


                    out_field.insert("object_type".to_owned(), Json::String(converted_type.to_html().to_owned()));

                    fields.push(Json::Object(out_field));

            }

            json_attribute.insert(
                "fields".to_owned(), 
                Json::Array(fields)
                );

            attributes.push(
                Json::Object(json_attribute)
            );
        }

        json.insert("attributes".to_owned(), attributes.into());

        Ok(Json::Object(json))

    }
    
    

}