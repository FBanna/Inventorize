use sqlx::{Pool, Sqlite, SqlitePool};




struct db {
    pool: Pool<Sqlite>
}


impl db{

    async fn new(path: &str) -> Self{

        Self{pool: SqlitePool::connect(path).await.unwrap()}
        
    }

}