use std::path::PathBuf;

use axum_typed_multipart::FieldData;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{config::config::Config, error::error::AppError};



pub struct DownloadedFile {
    pub component_id: Uuid,
    pub file_field: FieldData<NamedTempFile>
}