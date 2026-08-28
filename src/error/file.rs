use std::{error, fmt::{Display, format}};

use axum::{http::StatusCode, response::IntoResponse};



#[derive(Debug, Clone)]
pub enum FileErrors {

    Upload(String),
    WriteUpload,
    MimeUpload,
    ManipulateImageUpload

}

impl std::error::Error for FileErrors {}

impl Display for FileErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileErrors::Upload(template) => write!(f, "[ERROR] FileError - Upload - Failed to upload file: {}", template),
            FileErrors::WriteUpload => write!(f, "[ERROR] FileError - WriteUpload - Failed to write uploaded file"),
            FileErrors::MimeUpload => write!(f, "[ERROR] FileError - MimeUpload - Failed to infer mime type from file"),
            FileErrors::ManipulateImageUpload => write!(f, "[ERROR] FileError - ManipulateImageUpload - Failed to resize or compress image")
        }
    }
}

impl IntoResponse for FileErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}