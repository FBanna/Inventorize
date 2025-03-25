use sqlx::{migrate::{MigrateDatabase, Migrator}, Pool, Sqlite, SqlitePool};

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct Component {
    pool: Pool<Sqlite>
}


impl Component {

    pub async fn init(path: &str) -> Self{

        Self::create(path).await;

        let pool = SqlitePool::connect(path).await.unwrap();

        println!("IM HERE!! {:?}", MIGRATOR);

        let _ = MIGRATOR.run(&pool).await;

        Self{pool}
    
    }

    pub async fn create(path: &str){
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

}


