use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{config::config::Config, db::{component::properties::file::file::ComponentFile, db::DB}, error::error::AppError, server::db_api::files::get_file::get_file};




pub trait ComponentFileService {

    async fn add_file(&self, file: ComponentFile) -> Result<(), AppError>;

    async fn add_file_get_id(&self, name: String, mime: String, c_id: Uuid) -> Result<Uuid, AppError>;
    async fn del_file(&self, uuid: Uuid) -> Result<(), AppError>;
    async fn get_file(&self, uuid: Uuid) -> Result<ComponentFile, AppError>;
    async fn get_component_files(&self, c_id: Uuid) -> Result<Vec<ComponentFile>, AppError>;

    async fn get_all_file_clean_up_queue_id(&self) -> Result<Vec<FileCleanUp>, AppError>;
    async fn remove_id_from_file_clean_up_queue(&self, file_id: Uuid) -> Result<(), AppError>;

}

#[derive(FromRow)]
pub struct FileCleanUp {
    pub file_id: Uuid
}


impl ComponentFileService for DB {


    async fn add_file(&self, file: ComponentFile) -> Result<(), AppError> {
        
        let _output = sqlx::query("INSERT INTO component_file (file_id, component_id, name, mime) VALUES ($1, $2, $3, $4)")
            .bind(file.file_id)
            .bind(file.component_id)
            .bind(file.name)
            .bind(file.mime)
            .execute(&*self.pool)
            .await?;

        Ok(())


    }

    async fn add_file_get_id(&self, name: String, mime: String, c_id: Uuid) -> Result<Uuid, AppError> {
        
        let id = sqlx::query_scalar("INSERT INTO component_file (component_id, name, mime) VALUES ($1, $2, $3) RETURNING file_id")
            .bind(c_id)
            .bind(name)
            .bind(mime)
            .fetch_one(&*self.pool)
            .await?;

        Ok(id)

    }

    async fn del_file(&self, uuid: Uuid) -> Result<(), AppError> {
        
        let _output = sqlx::query("DELETE FROM component_file WHERE file_id = ($1)")
            .bind(uuid)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    async fn get_file(&self, uuid: Uuid) -> Result<ComponentFile, AppError> {
        
        let output: ComponentFile = sqlx::query_as("SELECT * FROM component_file WHERE file_id = ($1)")
            .bind(uuid)
            .fetch_one(&*self.pool)
            .await?;

        Ok(output)
    }

    async fn get_component_files(&self, c_id: Uuid) -> Result<Vec<ComponentFile>, AppError> {
        
        let output: Vec<ComponentFile> = sqlx::query_as("SELECT * FROM component_file WHERE component_id = ($1)")
            .bind(c_id)
            .fetch_all(&*self.pool)
            .await?;

        Ok(output)
    }


    
    async fn get_all_file_clean_up_queue_id(&self) -> Result<Vec<FileCleanUp>, AppError> {
        
        let ids: Vec<FileCleanUp> = sqlx::query_as("SELECT * FROM file_cleanup_queue")
            .fetch_all(&*self.pool)
            .await?;

        Ok(ids)

    }

    async fn remove_id_from_file_clean_up_queue(&self, file_id: Uuid) -> Result<(), AppError> {

        let result = sqlx::query("DELETE FROM file_cleanup_queue WHERE file_id = ($1)")
            .bind(file_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
    
    
    
    

}