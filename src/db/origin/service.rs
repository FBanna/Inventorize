use uuid::Uuid;

use crate::{db::{db::DB, origin::{origin::Origin, transport_origin::TransportOrigin}}, error::error::AppError};




pub trait OriginServices {

    async fn add_transport_origin(&self, to: TransportOrigin) -> Result<Uuid, AppError>;

    async fn get_all_origins(&self) -> Result<Vec<Origin>, AppError>;
    

}

impl OriginServices for DB {

    async fn add_transport_origin(&self, to: TransportOrigin) -> Result<Uuid, AppError> {
        
        let id: Uuid = sqlx::query_scalar("INSERT INTO origin(name, url, hurl_get, hurl_price) VALUES ($1,$2, $3, $4) RETURNING origin_id")
            .bind(&to.name)
            .bind(&to.url)
            .bind(&to.hurl_get)
            .bind(&to.hurl_price)
            .fetch_one(&*self.pool)
            .await?;

        Ok(id)

    }
    
    async fn get_all_origins(&self) -> Result<Vec<Origin>, AppError> {
        
        let origins: Vec<Origin> = sqlx::query_as("SELECT * FROM origin")
            .fetch_all(&*self.pool)
            .await?;

        Ok(origins)
    }

}