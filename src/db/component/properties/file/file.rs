use std::{fs, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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
        let option_mime = infer::get_from_path(&file.file_path).map_err(|_| FileErrors::WriteUpload)?;

        let mime = option_mime.ok_or(FileErrors::WriteUpload)?;


        let final_path = Path::new(&config.asset_location)
            .join(file.component_id.as_hyphenated().to_string())
            .join({
                
                format!("{}.{}", &file.temp_uuid.as_hyphenated().to_string(), &mime.extension())

            });
        

        fs::rename(file.file_path, final_path)?;

        

        Ok(Self {
            file_id: file.temp_uuid,
            component_id: file.component_id,
            name: file.name,
            mime: mime.mime_type().to_owned()
        })
    }

    // pub fn add_from_temp_file(c_id: Uuid, uuid: Uuid, temp_path: PathBuf, file_name: String, config: &Config) -> Result<Self, AppError> {

    //     let option_mime = infer::get_from_path(&temp_path).map_err(|_| FileError::WriteUpload)?;

    //     let mime = option_mime.ok_or(FileError::WriteUpload)?;


    //     let final_path = Path::new(&config.asset_location)
    //         .join(c_id.to_string())
    //         .join({
                
    //             format!("{}.{}", &uuid.as_hyphenated().to_string(), &mime.extension())

    //         });
        

    //     fs::rename(temp_path, final_path)?;

        

    //     Ok(Self {
    //         file_id: uuid,
    //         component_id: c_id,
    //         name: file_name,
    //         mime: mime.mime_type().to_owned()
    //     })
        

    // }

}


