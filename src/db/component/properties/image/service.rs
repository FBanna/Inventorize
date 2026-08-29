use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{db::{component::properties::image::image::ComponentImage, db::DB}, error::error::AppError};


pub trait ComponentImageService {


    async fn add_img(&self, img: ComponentImage) -> Result<(), AppError>;
    async fn del_img(&self, c_id: Uuid) -> Result<(), AppError>;
    async fn get_img(&self, c_id: Uuid) -> Result<ComponentImage, AppError>;
    async fn get_full(&self, c_id: Uuid) -> Result<Option<Vec<u8>>, AppError>;
    async fn get_thumb(&self, c_id: Uuid) -> Result<Option<Vec<u8>>, AppError>;

}

impl ComponentImageService for DB {


    // IMAGES

    async fn add_img(&self, img: ComponentImage) -> Result<(), AppError> {
        

        let _output = sqlx::query("INSERT INTO component_image (component_id, full_img, thumb_img) VALUES ($1, $2, $3)")
            .bind(img.component_id)
            .bind(img.full)
            .bind(img.thumb)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn del_img(&self, c_id: Uuid) -> Result<(), AppError> {
        
        let _output = sqlx::query("DELETE FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_img(&self, c_id: Uuid) -> Result<ComponentImage, AppError> {
        
        let output: ComponentImage = sqlx::query_as("SELECT * FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output)

    }

    async fn get_full(&self, c_id: Uuid) -> Result<Option<Vec<u8>>, AppError> {
        
        let output: Option<Vec<u8>>  = sqlx::query_scalar("SELECT (full_img) FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(output)

    }

    async fn get_thumb(&self, c_id: Uuid) -> Result<Option<Vec<u8>>, AppError> {
        
        let output: Option<Vec<u8>> = sqlx::query_scalar("SELECT (thumb_img) FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(output)


    }
}