use std::{fs, io::Cursor, path::{Path, PathBuf}};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateDatabase, Migrator}, prelude::FromRow, sqlite::{SqliteQueryResult, SqliteRow, SqliteValueRef}, ColumnIndex, Execute, Pool, QueryBuilder, Row, Sqlite, SqlitePool};

use crate::{config::config::Config, error::{self, error::AppError}};

use super::{db::DB, prompt::service::PromptServices, transport::post_component::PostComponent};


pub const ELEMENTS: [&str;6] = ["name","size","value","info","origin","label"];


// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Component{
//     pub id: Option<i32>,
//     pub name: String,
//     pub size: Option<String>,
//     pub value: Option<String>,
//     pub info: Option<String>,
//     pub stock: i32,
//     pub origin: Option<String>,
//     pub label: Option<String>,
//     pub image: Option<Vec<u8>>,
//     pub datasheet: Option<Vec<u8>>
// }

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Component{
    pub id: Option<i32>,
    pub name: String,
    pub size: Option<String>,
    pub value: Option<String>,
    pub info: Option<String>,
    pub stock: i32,
    pub origin: Option<String>,
    pub label: Option<String>,
    pub image: bool,
    pub datasheet: bool
}




impl Component{

    pub fn fmt(&self) -> String{
        return self.name.clone() + &self.size.clone().unwrap_or_else(|| {"none".to_string()}).clone();
    }

    pub fn to_vec(&self) -> Vec<Option<&str>> {

        vec![
            Some(self.name.as_str()),
            self.size.as_deref(),
            self.value.as_deref(),
            self.info.as_deref(),
            self.origin.as_deref(),
            self.label.as_deref(),
        ]

    } 

}


pub trait ComponentServices {
    async fn add_with_files(&self, c: PostComponent, config: &Config) -> Result<(), AppError>;

    async fn update_with_files(&self, id: i64, c: PostComponent, config: &Config) -> Result<(), AppError>;

    async fn add(&self, c: &Component) -> Result<SqliteQueryResult, AppError>;

    async fn update(&self, id: i64, c: &Component) -> Result<SqliteQueryResult, AppError>;

    async fn get_first(&self)  -> Result<Component, AppError>;

    async fn get_all(&self) -> Result<Vec<Component>, AppError>;

    async fn get(&self, i: i32) -> Result<Component, AppError>;

    async fn get_from_list(&self, list: Vec<i32>) -> Result<Vec<Component>, AppError>;

    async fn search(&self, c: Vec<Vec<String>>) -> Result<Vec<Component>, AppError>;

    async fn remove(&self, i: i32) -> Result<SqliteQueryResult, AppError>;   

    async fn remove_list(&self, list: Vec<i32>) -> Result<(), AppError>;

}


impl ComponentServices for DB{

    async fn remove(&self, i: i32) -> Result<SqliteQueryResult, AppError>{

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
            DELETE FROM components
            WHERE ROWID = (?)
        ").bind(i)
        .execute(&*self.pool).await?;

        Ok(result)
    }

    async fn remove_list(&self, list: Vec<i32>) -> Result<(), AppError> {

        for i in list{
            self.remove(i).await?;
        }

        Ok(())
    }

    async fn update_with_files(&self, id: i64, mut c: PostComponent, config: &Config) -> Result<(), AppError>{

        //c.update_component_file_bools();

        c.optimise_image();

        self.update(id,&c.component).await?;

        c.create_assets(id.into(), config);

        return Ok(());

    }

    async fn add_with_files(&self, mut c: PostComponent, config: &Config)  -> Result<(), AppError>{

        c.update_component_file_bools();

        c.optimise_image();

        let result: SqliteQueryResult = self.add(&c.component).await?;

        c.create_assets(result.last_insert_rowid(), config);

        return Ok(())

    }

    async fn update(&self, id: i64, c: &Component) -> Result<SqliteQueryResult, AppError> {
        
        let result: SqliteQueryResult = sqlx::query("
            UPDATE components
            SET
                name = (?),
                size = (?),
                value = (?),
                info = (?),
                stock = (?),
                origin = (?),
                label = (?),
                image = (?),
                datasheet = (?)
            WHERE
                ROWID = (?)
            ")
            .bind(&c.name)
            .bind(&c.size)
            .bind(&c.value) 
            .bind(&c.info)
            .bind(&c.stock)
            .bind(&c.origin)
            .bind(&c.label)
            .bind(&c.image)
            .bind(&c.datasheet)
            .bind(id)
            .execute(&*self.pool)
            .await?;

        Ok(result)

    }

    
    
    async fn add(&self, c: &Component) -> Result<SqliteQueryResult, AppError> {

        let result: SqliteQueryResult = sqlx::query("INSERT INTO components (name,size,value,info,stock,origin,label,image,datasheet) VALUES (?,?,?,?,?,?,?,?,?)")
            //.bind(c.ID)
            .bind(&c.name)
            .bind(&c.size)
            .bind(&c.value) 
            .bind(&c.info)
            .bind(&c.stock)
            .bind(&c.origin)
            //.bind(c.url)
            .bind(&c.label)
            .bind(&c.image)
            .bind(&c.datasheet)
            .execute(&*self.pool)
            .await?;


        self.update_prompts_add(&c).await;

        Ok(result)
    }

    // async fn get_first(&self) -> Result<{
    //     sqlx::query_as("SELECT * FROM components ORDER BY ROWID ASC LIMIT 1")
    //         .fetch_one(&self.pool)
    //         .await.unwrap();


    // }
    async fn get_first(&self) -> Result<Component, AppError>{
        let result: Component = sqlx::query_as("SELECT * FROM components ORDER BY ROWID ASC LIMIT 1")
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)



    }


    async fn get_all(&self) -> Result<Vec<Component>, AppError>{

        let result: Vec<Component> = sqlx::query_as("SELECT * FROM components")
            .fetch_all(&*self.pool)
            .await?;

        Ok(result)

    }
    

    async fn get(&self, i: i32) -> Result<Component, AppError> {
        let result: Component = sqlx::query_as("SELECT * FROM components WHERE id = (?)")
            .bind(i)
            .fetch_one(&*self.pool)
            .await?;

        Ok(result)
    }

    async fn get_from_list(&self, list: Vec<i32>) -> Result<Vec<Component>, AppError> {

        let mut result: Vec<Component> = Vec::new();

        println!("pulling from db");

        for i in list {

            let component: Component = sqlx::query_as("SELECT * FROM components WHERE id = (?)")
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

        let mut query: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT * FROM components WHERE ");

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


// pub struct TransportComponent{
//     pub id: Option<i32>,
//     pub name: String,
//     pub size: Option<String>,
//     pub value: Option<String>,
//     pub info: Option<String>,
//     pub stock: i32,
//     pub origin: Option<String>,
//     pub label: Option<String>,
//     pub image: Option<Vec<u8>>,
//     pub datasheet: Option<Vec<u8>>
// }

// impl TransportComponent {
//     pub fn into(&self) -> Component {
//         Component { 
//             id: self.id.clone(),
//             name: self.name.clone(), 
//             size: self.size.clone(), 
//             value: self.value.clone(), 
//             info: self.info.clone(), 
//             stock: self.stock, 
//             origin: self.origin.clone(), 
//             label: self.label.clone(), 
//             image: self.image.is_some(), 
//             datasheet: self.datasheet.is_some()
//         }
//     }



//     pub fn 


// }