use std::{fs::self, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{config::config::Config, db::component::properties::files::DownloadedFile, error::{error::AppError, file::FileErrors}};


#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ComponentFile {
    pub file_id: Uuid,
    pub component_id: Uuid,
    pub name: String,
    pub mime: String
}

impl ComponentFile {

    pub fn new(file: DownloadedFile, config: &Config) -> Result<Self, AppError> {



        let option_mime = infer::get_from_path(file.file_field.contents.path()).map_err(|_| FileErrors::MimeUpload)?;

        let mime = option_mime.ok_or(FileErrors::MimeUpload)?;

        let file_id = Uuid::now_v7();

        let out_dir = Path::new(&config.asset_location).join(file_id.as_hyphenated().to_string());

        file.file_field.contents.persist(out_dir).map_err(|_| FileErrors::Upload(
            format!("Could not find new file location, check {} dir exists!", config.asset_location)
        ))?;


        

        Ok(Self {
            file_id: file_id,
            component_id: file.component_id,
            name: file.file_field.metadata.file_name.unwrap_or("document".to_string()),
            mime: mime.mime_type().to_owned()
        })
    }

    pub async fn fetch_file_stream(&self, config: &Config) -> Result<ReaderStream<File>, AppError> {
        
        let path = PathBuf::from(config.asset_location.clone()).join(self.file_id.as_hyphenated().to_string());

        let file = File::open(path).await.map_err(|_| AppError::FileError(FileErrors::UnableToOpen))?;

        let stream: ReaderStream<File> = ReaderStream::new(file);
        

        Ok(stream)
    }


    #[deprecated]
    pub async fn remove(&self, config: &Config) -> Result<(), AppError> {
        let path = PathBuf::from(config.asset_location.clone()).join(self.file_id.as_hyphenated().to_string());

        fs::remove_file(path).map_err(|_| FileErrors::UnableToDelete.into())
    }

}


