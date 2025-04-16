use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateDatabase, Migrator}, prelude::FromRow, sqlite::SqliteRow, Pool, Row, Sqlite, SqlitePool};

use super::{db::DB, prompt::service::PromptServices};




#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Component{
    pub id: Option<i32>,
    pub name: String,
    pub size: Option<String>,
    pub value: Option<String>,
    pub info: Option<String>,
    pub stock: i32,
    pub origin: Option<String>,
    pub label: Option<String>
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
            self.label.as_deref()
        ]

    }
}


pub trait ComponentServices {
    async fn add(&self, c: Component);

    async fn get_first(&self) -> Component;

    async fn get_all(&self) -> Vec<Component>;

    async fn get(&self, i: i32) -> Component;

    async fn search(&self, c: Component) -> Vec<Component>;
}


impl ComponentServices for DB{
    
    async fn add(&self, c: Component){
        sqlx::query("INSERT INTO components (name,size,value,info,stock,origin,label) VALUES (?,?,?,?,?,?,?)")
            //.bind(c.ID)
            .bind(&c.name)
            .bind(&c.size)
            .bind(&c.value) 
            .bind(&c.info)
            .bind(&c.stock)
            .bind(&c.origin)
            //.bind(c.url)
            .bind(&c.label)
            .execute(&self.pool)
            .await
            .unwrap();

        self.update_prompts(c).await;
    }

    async fn get_first(&self) -> Component{
        sqlx::query_as("SELECT * FROM components ORDER BY ROWID ASC LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    async fn get_all(&self) -> Vec<Component>{
        sqlx::query_as("SELECT * FROM components")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
    

    async fn get(&self, i: i32) -> Component {
        sqlx::query_as("SELECT * FROM components WHERE id = (?)")
            .bind(i)
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    async fn search(&self, c: Component) -> Vec<Component> {

        sqlx::query_as("
            SELECT * FROM components
            WHERE name LIKE (?) AND
            stock > (?)
        ")
        .bind(c.name)
        // .bind(c.size)
        // .bind(c.value)
        .bind(c.stock)
        // .bind(c.origin)
        // .bind(c.label)
        .fetch_all(&self.pool)
        .await
        .unwrap()

    }

    // // GET ALL UNIQUE ENTRIES IN A COLUMN EG. RESISTOR, CAPACITOR
    // pub async fn get_component_prompts(&self) -> ComponentPrompts{

    //     sqlx::query_as("
    //         SELECT DISTINCT * FROM components
    //     ")
    //     //.bind(column)
    //     .fetch_all(&self.pool)
    //     .await
    //     .unwrap()
    // }

}