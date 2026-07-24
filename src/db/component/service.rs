
use std::{format, path::Ancestors, println};

use sqlx::{ColumnIndex, Execute, Pool, QueryBuilder, Row, Postgres, PgPool, migrate::{MigrateDatabase, Migrator}, prelude::FromRow, postgres::{PgQueryResult, PgRow, PgValueRef}, types::{Json, JsonRawValue}};
use uuid::Uuid;

use crate::{config::config::Config, db::{class::{class::Class, service::ClassServices}, class_instance::service::ClassInstanceServices, component::{component::Component, transport_component::TransportComponent}, component_class::{component_class::ComponentClass, service::ComponentClassServices}, db::DB}, error::{self, error::AppError, json::JsonErrors}};


pub trait ComponentServices {



    #[warn(deprecated)]
    async fn add_component(&self, c: &Component) -> Result<Uuid, AppError>;
    async fn add_transport_component(&self, c: &TransportComponent) -> Result<Uuid, AppError>;

    async fn update_component(&self, component_id: Uuid, c: &Component) -> Result<PgQueryResult, AppError>;

    async fn get_component(&self, component_id: Uuid) -> Result<Component, AppError>;
    async fn get_component_list(&self, list: Vec<Uuid>) -> Result<Vec<Component>, AppError>;

    //async fn search_component(&self, c: Vec<Vec<String>>) -> Result<Vec<Component>, AppError>;

    async fn remove_component(&self, component_id: Uuid, config: &Config) -> Result<PgQueryResult, AppError>;   

    async fn remove_component_list(&self, list: Vec<Uuid>, config: &Config) -> Result<(), AppError>;


    // async fn add_component_type_value(&self, tc: ComponentTypeValue) -> Result<PgQueryResult, AppError>;
    // async fn add_component_type_values(&self, tcs: Vec<ComponentTypeValue>) -> Result<(), AppError>;

    // async fn get_component_type_value(&self, c_id: i64, t_id: i64) -> Result<ComponentTypeValue, AppError>;
    // async fn get_component_type_values_t_id(&self, t_id: i64) -> Result<Vec<ComponentTypeValue>, AppError>;
    // async fn get_component_type_values_c_id(&self, c_id: i64) -> Result<Vec<ComponentTypeValue>, AppError>;


}


impl ComponentServices for DB{


    /// Delete a component + remove files
    async fn remove_component(&self, component_id: Uuid, config: &Config) -> Result<PgQueryResult, AppError>{


        let result = sqlx::query("
            DELETE FROM component
            WHERE component_id = ($1)
        ").bind(component_id)
        .execute(&*self.pool).await?;

        //remove_component_files(component_id, &config.asset_location); RE ADD THIS

        Ok(result)
    }

    async fn remove_component_list(&self, list: Vec<Uuid>, config: &Config) -> Result<(), AppError> {

        for i in list{
            self.remove_component(i, config).await?;
        }

        Ok(())
    }
    
    /// cant change component class_instance
    async fn update_component(&self, component_id: Uuid, c: &Component) -> Result<PgQueryResult, AppError> {
        
        let result: PgQueryResult = sqlx::query("
            UPDATE component
            SET
                name = ($1),
                stock = ($2),
                manufacturer = ($3),
                label = ($4),


            WHERE
                component_id = ($5)
            ")
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.manufacturer)
            .bind(&c.label)
            
            .bind(component_id)
            .execute(&*self.pool)
            .await?;

        Ok(result)

    }

    
    /// DEPRECATED
    async fn add_component(&self, c: &Component) -> Result<Uuid, AppError> {

        // component_type.veryify_attributes(&c.attributes)?;

        let id: Uuid = sqlx::query_scalar("INSERT INTO component (name,stock,manufacturer,label) VALUES ($1,$2,$3,$4) RETURNING component_id")
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.manufacturer)
            .bind(&c.label)
            .fetch_one(&*self.pool)
            .await?;



        Ok(id)
    }

    async fn add_transport_component(&self, c: &TransportComponent) -> Result<Uuid, AppError> {
        
        // retrieve class + class instance object
        let ancestors = self.get_class_ancestors_from_instance(c.class_instance_id).await?;

        // start transaction
        let mut tx = self.pool.begin().await?;

        // insert component
        let id: Uuid = sqlx::query_scalar("INSERT INTO component (class_instance_id,name,stock,manufacturer,label) VALUES ($1,$2,$3,$4,$5) RETURNING component_id")
            .bind(&c.class_instance_id)
            .bind(&c.name)
            .bind(&c.stock)
            .bind(&c.manufacturer)
            .bind(&c.label)

            .fetch_one(&mut *tx)
            .await?;

        // insert and verify attributes
        for ancestor in ancestors {

            let value = c.attributes
                    .get(&ancestor.class_id)
                    .ok_or(JsonErrors::ComponentClassAttributesMalformed(format!("Does not have attribute for {}", ancestor.name)))?;

            // verify
            Into::<Class>::into(ancestor.clone()).verify_component_attributes(value)?;

            // insert
            self.add_component_class(
                ComponentClass{
                    component_id: id,
                    class_instance_id: ancestor.class_instance_id,
                    attributes: value.to_owned()
                },
                &mut *tx
            ).await?;
        }

        // commit transaction
        tx.commit().await?;


        // handle other elements

        Ok(id)
    }


    

    async fn get_component(&self, component_id: Uuid) -> Result<Component, AppError> {

        // let result = sqlx::query_as("SELECT * FROM components WEHERE")


        let result: Component = sqlx::query_as("SELECT * FROM component WHERE component_id = ($1)")
            .bind(component_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)
    }
    
    async fn get_component_list(&self, list: Vec<Uuid>) -> Result<Vec<Component>, AppError> {
        
        let result: Vec<Component> = sqlx::query_as("SELECT * FROM component WHERE component_id in ($1)")
            .bind(list)
            .fetch_all(&*self.pool)
            .await?;

        Ok(result)
    }

    



}


// // why are you here?

// pub fn get_component_files(id: i32, name: &str, config: &str) -> Option<Vec<u8>> {
    
//     //let binding = config.to_owned() + "/" + &id.to_string() + "/" + name;


//     let asset_location = Path::new(config).join(id.to_string()).join(name);

//     println!("finding file {} at {}", name, asset_location.display());

//     if asset_location.exists() {

//         let result = fs::read(asset_location);

//         return result.ok()

//     }
//     None
// }



// pub fn write_component_files(id: i64, name: &str, config: &str, option: &Option<Vec<u8>>, is_present: bool) {

//     if is_present {
//         if let Some(data) = option {
//             //let binding = config.to_owned() + "\\" + &id.to_string();

            
//             let path: PathBuf = Path::new(config).join(id.to_string());

//             //println!("trying to access path at {}", path.as_os_str().to_str().get_or_insert_default());

//             if !path.exists() {
//                 fs::create_dir_all(&path).expect("could not create asset dir for component!");
//             }

//             fs::write(path.join(name.to_owned()), data).expect("Could not write asset file");

//         }

//     } else {

//         // THIS RUNS EVERY TIME YOU UPDATE A COMPONENT, LOTS OF SYS CALLS. COULD ADD
//         // ANOTHER PARAMETER TO REMOVE CERTAIN DATA FILES
//         let path: PathBuf = Path::new(config).join(id.to_string()).join(name.to_owned());

//         if path.exists(){
//             fs::remove_file(path).expect("could not remove file");
//         }
//     }

// }