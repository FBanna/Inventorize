use std::{path::{Path, PathBuf}, string};

use uuid::Uuid;

use crate::{config::config::Config, error::error::AppError};



pub struct ComponentFile {
    pub file_id: Uuid,
    pub component_id: i64,
    pub name: String,
    pub mime: String
}


impl ComponentFile {

    pub fn add_from_temp_file(c_id: i64, temp_path: PathBuf, uuid: Uuid, config: &Config) -> Result<Self, AppError> {


        let final_path = Path::new(&config.asset_location)
            .join(String::from_utf8())
            .join({
                match file_type.ok_or(RestError::WriteUpload)?.as_str() {
                    "image" => "image."
                }
            });

        fs::rename(file_path.ok_or(RestError::WriteUpload)?, final_path);


        

    };

}


