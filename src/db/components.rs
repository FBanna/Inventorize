use std::{fs, io::Cursor, path::Path};

use image::{imageops::FilterType, GenericImageView, ImageDecoder, ImageReader};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateDatabase, Migrator}, prelude::FromRow, sqlite::{SqliteRow, SqliteValueRef}, ColumnIndex, Execute, Pool, QueryBuilder, Row, Sqlite, SqlitePool};

use crate::cli::config::Config;

use super::{db::DB, prompt::service::PromptServices};


pub const ELEMENTS: [&str;6] = ["name","size","value","info","origin","label"];


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component{
    pub id: Option<i32>,
    pub name: String,
    pub size: Option<String>,
    pub value: Option<String>,
    pub info: Option<String>,
    pub stock: i32,
    pub origin: Option<String>,
    pub label: Option<String>,
    pub image: Option<Vec<u8>>,
    pub datasheet: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ComponentDB{
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

    pub fn optimise_image(&mut self) {
        if let Some(some_image) = &self.image {
            let res_img_reader = ImageReader::new(Cursor::new(some_image)).with_guessed_format();

            if let Ok(img_reader) = res_img_reader{
                let res_img = img_reader.decode();

                if let Ok(img) = res_img {

                    let new_img;
                    
                    if img.dimensions() != (1000, 1000) {

                        new_img = img.resize(1000, 1000, FilterType::Nearest);

                    } else {
                        new_img = img;
                    }

                    let mut bytes: Vec<u8> = Vec::new();
                        
                    new_img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png);

                    self.image = Some(bytes);

                    println!("optimised!");

                    return;


                }
            }
        }

        println!("not optimised!");

        self.image = None;
    }

    fn create_assets(&self, id: i64, config: &Config) {
        write_files(id, "full.png", &config.asset_location, &self.image);
        write_files(id, "datasheet.pdf", &config.asset_location, &self.datasheet);
    }

}

impl ComponentDB {

    fn component(&self, config: &Config) -> Component {

        let id = unsafe { self.id.unwrap_unchecked() };
        
        let mut image: Option<Vec<u8>> = None;
        let mut datasheet: Option<Vec<u8>> = None;

        if self.image {
            println!("theres an image! just having a look now");

            image = find_component_files(id, "full.png", &config.asset_location);
        }

        if self.datasheet {
            println!("theres an image! just having a look now");

            datasheet = find_component_files(id, "datasheet", &config.asset_location);
        }

        return Component {
            id: self.id.clone(),
            name: self.name.clone(),
            size: self.size.clone(),
            value: self.value.clone(),
            info: self.info.clone(),
            stock: self.stock.clone(),
            origin: self.origin.clone(),
            label: self.label.clone(),
            image,
            datasheet,
        };

    }



}

// impl FromRow<'_, SqliteRow> for Component {
//     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {

//         let id: Option<i32> = row.try_get("id")?;

//         let image: Option<Vec<u8>>;
//         let datasheet: Option<Vec<u8>>;

//         if row.try_get("image")? {
//             println!("theres an image! just having a look now");

//             find_component_files(id, "full.png", config)

            

//         }
        
//         return Ok( Component {
//             id: id,
//             name: row.try_get("name")?,
//             size: row.try_get("size")?,
//             value: row.try_get("value")?,
//             info: row.try_get("info")?,
//             stock: row.try_get("stock")?,
//             origin: row.try_get("origin")?,
//             label: row.try_get("label")?,
//             image: row.try_get("value")?,
//             datasheet: row.try_get("value")?,
//         } );
//     }
// }

pub trait ComponentServices {
    async fn add(&self, c: Component, config: &Config);

    async fn get_first(&self, config: &Config) -> Component;

    async fn get_all(&self, config: &Config) -> Vec<Component>;

    async fn get(&self, i: i32, config: &Config) -> Component;

    async fn search(&self, c: Vec<Vec<String>>, config: &Config) -> Vec<Component>;

    

}


impl ComponentServices for DB{

    
    
    async fn add(&self, mut c: Component, config: &Config){

        c.optimise_image();
        

        let result = sqlx::query("INSERT INTO components (name,size,value,info,stock,origin,label,image,datasheet) VALUES (?,?,?,?,?,?,?,?,?)")
            //.bind(c.ID)
            .bind(&c.name)
            .bind(&c.size)
            .bind(&c.value) 
            .bind(&c.info)
            .bind(&c.stock)
            .bind(&c.origin)
            //.bind(c.url)
            .bind(&c.label)
            .bind(&c.image.is_some())
            .bind(&c.datasheet.is_some())
            .execute(&self.pool)
            .await
            .unwrap();

        

        c.create_assets(result.last_insert_rowid(), &config);



        self.update_prompts(c).await;
    }

    async fn get_first(&self, config: &Config) -> Component{
        let result: ComponentDB = sqlx::query_as("SELECT * FROM components ORDER BY ROWID ASC LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .unwrap();

        result.component(config)

    }

    async fn get_all(&self, config: &Config) -> Vec<Component>{
        let result: Vec<ComponentDB> = sqlx::query_as("SELECT * FROM components")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let mut components = Vec::new();

        for componentdb in result {
            components.push(componentdb.component(config));
        }

        return components;
        
    }
    

    async fn get(&self, i: i32, config: &Config) -> Component {
        let result: ComponentDB = sqlx::query_as("SELECT * FROM components WHERE id = (?)")
            .bind(i)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        result.component(config)
    }


    async fn search(&self, c: Vec<Vec<String>>, config: &Config) -> Vec<Component> {


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
            return Vec::new();
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

        let result: Vec<ComponentDB> = query.build_query_as::<ComponentDB>().fetch_all(&self.pool).await.unwrap();

        let mut components = Vec::new();

        for componentdb in result {
            components.push(componentdb.component(config));
        }

        return components;



    }




}


pub fn find_component_files(id: i32, name: &str, config: &str) -> Option<Vec<u8>> {
    
    let binding = config.to_owned() + "/" + &id.to_string() + name;
    let asset_location = Path::new(&binding);

    println!("finding file {} at {}", name, binding);

    if asset_location.exists() {

        let result = fs::read(asset_location);

        return result.ok()

    }
    None
}



fn write_files(id: i64, name: &str, config: &str, option: &Option<Vec<u8>>) {

    println!("im writing {}", name);

    if let Some(data) = option {
        let binding = config.to_owned() + "\\" + &id.to_string();

        
        let path = Path::new(&binding);

        println!("trying to access path at {}", path.as_os_str().to_str().get_or_insert_default());

        if !path.exists() {
            fs::create_dir_all(path).expect("could not create asset dir for component!");
        }

        fs::write(path.join(name.to_owned()), data).expect("Could not write asset file");

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