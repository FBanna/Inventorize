use std::sync::Arc;

use sqlx::{migrate::{MigrateDatabase, Migrator}, Pool, Sqlite, SqlitePool};

use super::prompt::{prompts::Prompts, service::PromptServices};



pub struct DB {
    pub pool: Arc<Pool<Sqlite>>,
    // could have a cached prompts stay open here
    pub prompt_cache: Prompts
}

static MIGRATOR: Migrator = sqlx::migrate!();

impl DB {

    async fn new(path: &str) -> Self {

        Self::create(path).await;

        let pool = SqlitePool::connect(path).await.unwrap();

        let _ = MIGRATOR.run(&pool).await;

        let prompt_cache = Prompts::new();

        

        Self{pool: Arc::new(pool), prompt_cache}

    }

    pub async fn init(path: &str) -> Self{

        let mut db = Self::new(path).await;

        db.sync_prompts().await;

        return db;
    
    }

    async fn create(path: &str){
        if !Sqlite::database_exists(path).await.unwrap_or(false) {
            //println!("Creating database {}", path);
            match Sqlite::create_database(path).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } 
        // else {
        //     println!("Database already exists");
        // }
    }
}