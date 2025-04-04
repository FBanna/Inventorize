use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateDatabase, Migrator}, prelude::FromRow, sqlite::SqliteRow, Pool, Row, Sqlite, SqlitePool};

static MIGRATOR: Migrator = sqlx::migrate!();


#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Component{
    //pub ID: Option<i32>,
    pub name: String,
    pub size: Option<String>,
    pub value: Option<String>,
    pub info: Option<String>,
    pub stock: i32,
    pub origin: Option<String>,
    pub url: Option<String>
}

// impl<'r> sqlx::FromRow<'r, SqliteRow> for Component {
//     fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
//         use Row;
//         let name = row.try_get("name")?;
//         let size = row.try_get("size");
//         let value = row.try_get("value");
//         let info = row.try_get("info");
//         let origin = row.try_get("origin");
//         let url = row.try_get("url");

//         Ok(Component{
//             name,
//             size:size,
//             value:value,
//             info:info,
//             stock,
//             origin:origin,
//             url:url
//         })
//     }
// }

impl Component{

    pub fn fmt(&self) -> String{
        return self.name.clone() + &self.size.clone().unwrap_or_else(|| {"none".to_string()}).clone();
    }
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

    pub async fn add(&self, c: Component){
        sqlx::query("INSERT INTO components (name,size,value,info,stock,origin,url) VALUES (?,?,?,?,?,?,?)")
            //.bind(c.ID)
            .bind(c.name)
            .bind(c.size)
            .bind(c.value) 
            .bind(c.info)
            .bind(c.stock)
            .bind(c.origin)
            .bind(c.url)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn get_first(&self) -> Component{
        sqlx::query_as("SELECT * FROM components ORDER BY ROWID ASC LIMIT 1")
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_all(&self) -> Vec<Component>{
        sqlx::query_as("SELECT * FROM components")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
    

}


