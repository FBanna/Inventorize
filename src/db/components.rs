use sqlx::{migrate::{MigrateDatabase, Migrator}, Pool, Sqlite, SqlitePool};

static MIGRATOR: Migrator = sqlx::migrate!();


//#[derive(sqlx::FromRow)]
pub struct Component{
    pub ID: i32,
    pub NAME: String,
    pub SIZE: Option<String>,
    pub INFO: Option<String>,
    pub STOCK: i32,
    pub ORIGIN: Option<String>,
    pub URL: Option<String>
}

pub struct Components {
    pool: Pool<Sqlite>
}


impl Components {

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

    pub async fn add(self, c: Component){
        sqlx::query("INSERT INTO components (ID,NAME,SIZE,INFO,STOCK,ORIGIN,URL) VALUES (?,?,?,?,?,?,?)")
            .bind(c.ID)
            .bind(c.NAME)
            .bind(c.SIZE)
            .bind(c.INFO)
            .bind(c.STOCK)
            .bind(c.ORIGIN)
            .bind(c.URL)
            .execute(&self.pool)
            .await
            .unwrap();
    }

}


