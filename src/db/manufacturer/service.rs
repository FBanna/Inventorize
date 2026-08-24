use uuid::Uuid;

use crate::{db::{db::DB, manufacturer::{self, manufacturer::Manufacturer, transport::TransportManufacturer}}, error::error::AppError};




pub trait ManufacturerServices {

    async fn add_transport_manufacturer(&self, tm: TransportManufacturer) -> Result<Uuid, AppError>;

    async fn get_manufacturer(&self, id: Uuid) -> Result<Manufacturer, AppError>;

    async fn get_all_manufacturer(&self) -> Result<Vec<Manufacturer>, AppError>;
}


impl ManufacturerServices for DB{
    async fn add_transport_manufacturer(&self, tm: TransportManufacturer) -> Result<Uuid, AppError> {

        let id: Uuid = sqlx::query_scalar("INSERT INTO manufacturer(name, url) VALUES ($1,$2) RETURNING manufacturer_id")
            .bind(&tm.name)
            .bind(&tm.url)
            .fetch_one(&*self.pool)
            .await?;

        Ok(id)
    }

    async fn get_manufacturer(&self, id: Uuid) -> Result<Manufacturer, AppError> {
        
        let manufacturer: Manufacturer = sqlx::query_as("SELECT * FROM manufacturer WHERE manufacturer_id = ($1)")
            .bind(&id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(manufacturer)

    }

    async fn get_all_manufacturer(&self) -> Result<Vec<Manufacturer>, AppError> {
        
        let manufacturers: Vec<Manufacturer> = sqlx::query_as("SELECT * FROM manufacturer")
            .fetch_all(&*self.pool)
            .await?;

        Ok(manufacturers)

    }
}

