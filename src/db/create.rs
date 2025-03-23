use sqlx::{migrate::MigrateDatabase, Sqlite};



pub async fn init(path: &str){

    if !Sqlite::database_exists(path).await.unwrap_or(false) {
        println!("Creating database {}", path);
        match Sqlite::create_database(path).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    
}