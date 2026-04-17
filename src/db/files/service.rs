use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{db::{db::DB, files::{file::file::ComponentFile, image::image::ComponentImage}}, error::error::AppError};

#[derive(FromRow)]
struct FullBytes {
    full: Vec<u8>
}


#[derive(FromRow)]
struct ThumbBytes {
    thumb: Vec<u8>
}

pub trait ComponentFileService {

    async fn add_file(&self, c_id: i64, file: ComponentFile) -> Result<(), AppError>;
    async fn del_file(&self, uuid: Uuid) -> Result<(), AppError>;
    async fn get_file(&self, uuid: Uuid) -> Result<ComponentFile, AppError>;
    async fn get_all_files(&self, c_id: i64) -> Result<Vec<ComponentFile>, AppError>;

    async fn add_img(&self, c_id: i64, img: ComponentImage) -> Result<(), AppError>;
    async fn del_img(&self, c_id: i64) -> Result<(), AppError>;
    async fn get_img(&self, c_id: i64) -> Result<ComponentImage, AppError>;
    async fn get_full(&self, c_id: i64) -> Result<Vec<u8>, AppError>;
    async fn get_thumb(&self, c_id: i64) -> Result<Vec<u8>, AppError>;

}

impl ComponentFileService for DB {



    // FILES

    async fn add_file(&self, c_id: i64, file: ComponentFile) -> Result<(), AppError> {
        
        let _output = sqlx::query("INSERT INTO component_file (file_id, component_id, name, mime) VALUES ($1, $2, $3, $4)")
            .bind(file.file_id)
            .bind(c_id)
            .bind(file.name)
            .bind(file.mime)
            .execute(&*self.pool)
            .await?;

        Ok(())


    }

    async fn del_file(&self, uuid: Uuid) -> Result<(), AppError> {
        
        let _output = sqlx::query("DELETE FROM component_file WHERE file_id = ($1)")
            .bind(uuid)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_file(&self, uuid: Uuid) -> Result<ComponentFile, AppError> {
        
        let output: ComponentFile = sqlx::query_as("SELECT (file_id, name, mime) FROM component_file WHERE file_id = ($1)")
            .bind(uuid)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output)
    }

    async fn get_all_files(&self, c_id: i64) -> Result<Vec<ComponentFile>, AppError> {
        
        let output: Vec<ComponentFile> = sqlx::query_as("SELECT (file_id, name, mime) FROM component_file WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(output)
    }



    // IMAGES

    async fn add_img(&self, c_id: i64, img: ComponentImage) -> Result<(), AppError> {
        

        let _output = sqlx::query("INSERT INTO component_image (component_id, full, thumb) VALUES ($1, $2, $3)")
            .bind(c_id)
            .bind(img.full)
            .bind(img.thumb)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn del_img(&self, c_id: i64) -> Result<(), AppError> {
        
        let _output = sqlx::query("DELETE FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_img(&self, c_id: i64) -> Result<ComponentImage, AppError> {
        
        let output: ComponentImage = sqlx::query_as("SELECT (full, thumb) FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output)

    }

    async fn get_full(&self, c_id: i64) -> Result<Vec<u8>, AppError> {
        
        let output: FullBytes  = sqlx::query_as("SELECT (full) FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output.full)

    }

    async fn get_thumb(&self, c_id: i64) -> Result<Vec<u8>, AppError> {
        
        let output: ThumbBytes  = sqlx::query_as("SELECT (thumb) FROM component_image WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output.thumb)


    }
}