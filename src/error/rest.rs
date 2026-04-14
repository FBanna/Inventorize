use std::{error, fmt::{Display, format}};

use axum::{http::StatusCode, response::IntoResponse};



#[derive(Debug, Clone)]
pub enum RestError {

    Upload(String),
    WriteUpload,

}

impl std::error::Error for RestError {}

impl Display for RestError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RestError::Upload(template) => write!(f, "[ERROR] RestError - Upload - Failed to upload file: {}", template),
            RestError::WriteUpload => write!(f, "[ERROR] RestError - WriteUpload - Failed to write uploaded file")

        }
    }
}

impl IntoResponse for RestError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}