use uuid::Uuid;

use crate::{db::{db::DB, origin::{origin::Origin, transport_origin::TransportOrigin}}, error::error::AppError};




pub trait OriginServices {

    async fn add_transport_origin(&self, to: TransportOrigin) -> Result<Uuid, AppError>;

    async fn get_all_origins(&self) -> Result<Vec<Origin>, AppError>;

    async fn get_origin(&self, origin_id: Uuid) -> Result<Origin, AppError>;
    

}

impl OriginServices for DB {

    async fn add_transport_origin(&self, to: TransportOrigin) -> Result<Uuid, AppError> {
        
        let id: Uuid = sqlx::query_scalar("INSERT INTO origin(name, url, price_hurl, hurl_pn, hurl_qr) VALUES ($1,$2, $3, $4, $5) RETURNING origin_id")
            .bind(&to.name)
            .bind(&to.url)
            .bind(&to.price_hurl)
            .bind(&to.hurl_pn)
            .bind(&to.hurl_qr)
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
    
    async fn get_origin(&self, origin_id: Uuid) -> Result<Origin, AppError> {
        
        let origin: Origin = sqlx::query_as("SELECT * FROM origin WHERE origin_id = ($1)")
            .bind(origin_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(origin)

    }

}