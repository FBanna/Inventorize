use std::{error, fmt::{Display, format}};

use axum::{http::StatusCode, response::IntoResponse};



#[derive(Debug, Clone)]
pub enum FileError {

    Upload(String),
    WriteUpload,

}

impl std::error::Error for FileError {}

impl Display for FileError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileError::Upload(template) => write!(f, "[ERROR] FileError - Upload - Failed to upload file: {}", template),
            FileError::WriteUpload => write!(f, "[ERROR] FileError - WriteUpload - Failed to write uploaded file")

        }
    }
}

impl IntoResponse for FileError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}