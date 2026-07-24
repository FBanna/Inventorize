use std::path::PathBuf;

use uuid::Uuid;

use crate::{config::config::Config, error::error::AppError};



pub struct DownloadedFile {
    pub name: String,
    pub component_id: Uuid,
    pub file_path: PathBuf,
    pub temp_uuid: Uuid
}