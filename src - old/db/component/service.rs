use std::{fs, io::Cursor, path::{Path, PathBuf}};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{ColumnIndex, Execute, Pool, QueryBuilder, Row, Postgres, PgPool, migrate::{MigrateDatabase, Migrator}, prelude::FromRow, postgres::{PgQueryResult, PgRow, PgValueRef}, types::{Json, JsonRawValue}};

use crate::{config::config::Config, db::{component::component::{Component, ELEMENTS}, db::DB, prompt::service::PromptServices, transport::transport_component::TransportComponent, types::{component_type_attributes, component_type_value::ComponentTypeValue, service::ComponentTypeService}}, error::{self, error::AppError, json::JsonError}};


pub trait ComponentServices {


    // async fn add_with_files(&self, c: TransportComponent, config: &Config) -> Result<(), AppError>;

    // async fn update_with_files(&self, id: i32, c: TransportComponent, config: &Config) -> Result<(), AppError>;

    #[warn(deprecated)]
    async fn add(&self, c: &Component) -> Result<i64, AppError>;

    async fn add_transport_component(&self, c: &TransportComponent) -> Result<i64, AppError>;

    async fn update(&self, id: i64, c: &Component) -> Result<PgQueryResult, AppError>;

    async fn get_first(&self)  -> Result<Component, AppError>;
    async fn get_all(&self) -> Result<Vec<Component>, AppError>; // UPDATE

    async fn get(&self, i: i64) -> Result<Component, AppError>;

    async fn get_from_list(&self, list: Vec<i64>) -> Result<Vec<Component>, AppError>;

    async fn search(&self, c: Vec<Vec<String>>) -> Result<Vec<Component>, AppError>;

    async fn remove(&self, i: i64, config: &Config) -> Result<PgQueryResult, AppError>;   

    async fn remove_list(&self, list: Vec<i64>, config: &Config) -> Result<(), AppError>;



    //async fn add_component_types(&self, c: &Component) -> Result<(), AppError>;

    async fn add_component_type_value(&self, tc: ComponentTypeValue) -> Result<PgQueryResult, AppError>;
    async fn add_component_type_values(&self, tcs: Vec<ComponentTypeValue>) -> Result<(), AppError>;

    async fn get_component_type_value(&self, c_id: i64, t_id: i64) -> Result<ComponentTypeValue, AppError>;
    async fn get_component_type_values_t_id(&self, t_id: i64) -> Result<Vec<ComponentTypeValue>, AppError>;
    async fn get_component_type_values_c_id(&self, c_id: i64) -> Result<Vec<ComponentTypeValue>, AppError>;


}


impl ComponentServices for DB{

    async fn remove(&self, i: i64, config: &Config) -> Result<PgQueryResult, AppError>{

        let c = self.get(i).await?;


        self.update_prompts_del(&c).await;

        // sqlx::query("
        //     DELETE FROM components
        //     WHERE ROWID = (?)
        // ").bind(i)
        // .execute(&self.pool)
        // .await
        // .unwrap();

        let result = sqlx::query("
            DELETE FROM component
            WHERE component_id = ($1)
        ").bind(i)
        .execute(&*self.pool).await?;

        remove_component_files(i, &config.asset_location);

        Ok(result)
    }

    async fn remove_list(&self, list: Vec<i64>, config: &Config) -> Result<(), AppError> {

        for i in list{
            self.remove(i, config).await?;
        }

        Ok(())
    }
    

    async fn update(&self, id: i64, c: &Component) -> Result<PgQueryResult, AppError> {


        let old = self.get(id).await?;

        
        
        let result: PgQueryResult = sqlx::query("
            UPDATE component
            SET
                name = ($1),
                stock = ($2),
                price = ($3),
                manufacturer = ($4),
                label = ($5),
                image = ($6),
                datasheet = ($7),

            WHERE
                component_id = ($8)
            ")
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.price)
            .bind(&c.manufacturer)
            .bind(&c.label)
            .bind(&c.image)
            .bind(&c.datasheet)
            //.bind(&c.attribute_id)
            //.bind(&c.attributes)
            .bind(id)
            .execute(&*self.pool)
            .await?;

        self.update_prompts_del(&old).await;
        self.update_prompts_add(&c).await;
        

        Ok(result)

    }

    async fn add_component_type_value(&self, tc: ComponentTypeValue) -> Result<PgQueryResult, AppError> {
        
        let component_type = self.get_type(tc.type_id).await?;

        component_type.get_attributes()?.veryify_attributes(&tc.attributes)?;

        


        let result: PgQueryResult = sqlx::query("INSERT INTO component_type (component_id, type_id, attributes) VALUES ($1,$2,$3)")
            .bind(&tc.component_id)
            .bind(&tc.type_id)
            .bind(&tc.attributes)
            .execute(&*self.pool)
            .await?;


        Ok(result)
    }

    async fn add_component_type_values(&self, tcs: Vec<ComponentTypeValue>) -> Result<(), AppError> {
        for tc in tcs {
            self.add_component_type_value(tc).await?;
        }

        Ok(())
    }

    async fn get_component_type_value(&self, c_id: i64, t_id: i64) -> Result<ComponentTypeValue, AppError> {
        let result: ComponentTypeValue = sqlx::query_as("
            SELECT * FROM component_type
            WHERE type_id = ($1)
            AND component_id = ($2)
        ")
        .bind(t_id)
        .bind(c_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn get_component_type_values_c_id(&self, c_id: i64) -> Result<Vec<ComponentTypeValue>, AppError> {
        
        let result: Vec<ComponentTypeValue> = sqlx::query_as("
            SELECT * FROM component_type
            WHERE component_id = ($1)
        ")
        .bind(c_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)
    }

    async fn get_component_type_values_t_id(&self, t_id: i64) -> Result<Vec<ComponentTypeValue>, AppError> {
        let result: Vec<ComponentTypeValue> = sqlx::query_as("
            SELECT * FROM component_type
            WHERE type_id = ($1)
        ")
        .bind(t_id)
        .fetch_all(&*self.pool)
        .await?;

        Ok(result)
    }
    
    /// DEPRECATED
    async fn add(&self, c: &Component) -> Result<i64, AppError> {



        //self.add_component_types(c).await?;

        // component_type.veryify_attributes(&c.attributes)?;

        let id: i64 = sqlx::query_scalar("INSERT INTO component (name,stock,price,manufacturer,label,image,datasheet) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING component_id")
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.price)
            .bind(&c.manufacturer)
            .bind(&c.label)
            .bind(&c.image)
            .bind(&c.datasheet)
            //.bind(&c.attributes)
            .fetch_one(&*self.pool)
            .await?;



        Ok(id)
    }

    async fn add_transport_component(&self, c: &TransportComponent) -> Result<i64, AppError> {

        let id: i64 = sqlx::query_scalar("INSERT INTO component (name,stock,price,manufacturer,label,image,datasheet) VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING component_id")
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.price)
            .bind(&c.manufacturer)
            .bind(&c.label)
            .bind(&c.image)
            .bind(&c.datasheet)
            //.bind(&c.attributes)
            .fetch_one(&*self.pool)
            .await?;



        Ok(id)
    }


    async fn get_first(&self) -> Result<Component, AppError>{
        
        let result: Component = sqlx::query_as("SELECT * FROM component ORDER BY component_id ASC LIMIT 1")
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)



    }


    async fn get_all(&self) -> Result<Vec<Component>, AppError>{

        let result: Vec<Component> = sqlx::query_as("SELECT * FROM component")
            .fetch_all(&*self.pool)
            .await?;

        Ok(result)

    }
    

    async fn get(&self, i: i64) -> Result<Component, AppError> {

        // let result = sqlx::query_as("SELECT * FROM components WEHERE")


        let result: Component = sqlx::query_as("SELECT * FROM component WHERE component_id = ($1)")
            .bind(i)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)
    }

    async fn get_from_list(&self, list: Vec<i64>) -> Result<Vec<Component>, AppError> {

        let mut result: Vec<Component> = Vec::new();

        println!("pulling from db");

        for i in list {

            let component: Component = sqlx::query_as("SELECT * FROM component WHERE component_id = ($1)")
                .bind(i)
                .fetch_one(&*self.pool)
                .await?;

            //if let Ok(compnent) = component_result {

                result.push(component);
            // } else {
            //     return Err(component_result.err().unwrap())
            // }     
        }

        println!("finished pulling");

        return Ok(result);

    }


    async fn search(&self, c: Vec<Vec<String>>) -> Result<Vec<Component>, AppError> {


        let mut emptied = Vec::new();

        // EMPTY INPUT
        for (i, element) in c.into_iter().enumerate() {
            if !element.is_empty(){
                emptied.push((element, ELEMENTS[i]));
            }
        }

        let len = emptied.len();


        // RETURN IF NOTHING TO SEARCH
        if len == 0 {
            return self.get_all().await;
        }

        // BUILD QUERY

        let mut query: QueryBuilder<Postgres> = QueryBuilder::new("SELECT * FROM component WHERE ");

        for (index, list) in emptied.into_iter().enumerate() {

            query.push(list.1.to_owned() + " IN (");

            let mut list_query = query.separated(",");

            for value in list.0 {
                list_query.push_bind(value);
            }

            if len-1 == index {
                
                query.push(")");
            } else {
                query.push(") AND ");
            }
            
        }

        let result: Vec<Component> = query.build_query_as::<Component>().fetch_all(&*self.pool).await?;

        Ok(result)



    }




}


pub fn get_component_files(id: i32, name: &str, config: &str) -> Option<Vec<u8>> {
    
    //let binding = config.to_owned() + "/" + &id.to_string() + "/" + name;


    let asset_location = Path::new(config).join(id.to_string()).join(name);

    println!("finding file {} at {}", name, asset_location.display());

    if asset_location.exists() {

        let result = fs::read(asset_location);

        return result.ok()

    }
    None
}

/// NEED TO CHANGE
pub fn remove_component_files(id: i64, config: &str) {

    let path: PathBuf = Path::new(config).join(id.to_string());

    if path.exists() {
        fs::remove_dir(path).expect("could not delete folder");
    }

}


pub fn write_component_files(id: i64, name: &str, config: &str, option: &Option<Vec<u8>>, is_present: bool) {

    if is_present {
        if let Some(data) = option {
            //let binding = config.to_owned() + "\\" + &id.to_string();

            
            let path: PathBuf = Path::new(config).join(id.to_string());

            //println!("trying to access path at {}", path.as_os_str().to_str().get_or_insert_default());

            if !path.exists() {
                fs::create_dir_all(&path).expect("could not create asset dir for component!");
            }

            fs::write(path.join(name.to_owned()), data).expect("Could not write asset file");

        }

    } else {

        // THIS RUNS EVERY TIME YOU UPDATE A COMPONENT, LOTS OF SYS CALLS. COULD ADD
        // ANOTHER PARAMETER TO REMOVE CERTAIN DATA FILES
        let path: PathBuf = Path::new(config).join(id.to_string()).join(name.to_owned());

        if path.exists(){
            fs::remove_file(path).expect("could not remove file");
        }
    }

}