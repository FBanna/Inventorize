use std::sync::Arc;

use sqlx::{ConnectOptions, PgPool, Pool, Postgres, migrate::{MigrateDatabase, Migrator}};


pub struct DB {
    pub pool: Arc<Pool<Postgres>>,
    // could have a cached prompts stay open here
    //pub prompt_cache: Prompts
}

static MIGRATOR: Migrator = sqlx::migrate!("db/migrations");

impl DB {

    async fn new(path: &str) -> Self {

        Self::create(path).await;

        let pool = PgPool::connect(path).await.unwrap();
        
        MIGRATOR.run(&pool).await.expect("MIGRATION ERROR");

        // if result.is_err(){
        //     println!("MIGRATION ERROR: {}", result.err().unwrap().to_string())

        // }

        //let prompt_cache = Prompts::new();

        

        Self{pool: Arc::new(pool)}

    }

    pub async fn init(path: &str) -> Self{

        let mut db = Self::new(path).await;

        //db.sync_prompts().await;

        return db;
    
    }

    pub async fn create(path: &str){
        if !Postgres::database_exists(path).await.unwrap_or(false) {
            //println!("Creating database {}", path);
            match Postgres::create_database(path).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } 
        // else {
        //     println!("Database already exists");
        // }
    }
}