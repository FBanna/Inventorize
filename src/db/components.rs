use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateDatabase, Migrator}, prelude::FromRow, sqlite::SqliteRow, Execute, Pool, QueryBuilder, Row, Sqlite, SqlitePool};

use super::{db::DB, prompt::service::PromptServices};


pub const ELEMENTS: [&str;6] = ["name","size","value","info","origin","label"];


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

    async fn search(&self, c: Vec<Vec<String>>) -> Vec<Component>;
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


    async fn search(&self, c: Vec<Vec<String>>) -> Vec<Component> {


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

        query.build_query_as::<Component>().fetch_all(&self.pool).await.unwrap()

    }


}