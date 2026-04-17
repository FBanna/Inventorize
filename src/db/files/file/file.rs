use std::{fs, path::{Path, PathBuf}, string};
use infer::Type as MimeType;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{config::config::Config, error::{error::AppError, file::FileError}};


#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ComponentFile {
    pub file_id: Uuid,
    // pub component_id: i64,
    pub name: String,
    pub mime: String
}


impl ComponentFile {

    pub fn add_from_temp_file(c_id: i64, uuid: Uuid, temp_path: PathBuf, file_name: String, config: &Config) -> Result<Self, AppError> {

        let option_mime = infer::get_from_path(&temp_path).map_err(|_| FileError::WriteUpload)?;

        let mime = option_mime.ok_or(FileError::WriteUpload)?;


        let final_path = Path::new(&config.asset_location)
            .join(c_id.to_string())
            .join({
                
                format!("{}.{}", &uuid.as_hyphenated().to_string(), &mime.extension())

            });
        

        fs::rename(temp_path, final_path)?;

        

        Ok(Self {
            file_id: uuid,
            //component_id: c_id,
            name: file_name,
            mime: mime.mime_type().to_owned()
        })
        

    }

}


