use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum LabelError {

    MissingTemplate(String),
    Compilation(),
    Export()

}


impl std::error::Error for LabelError {}



impl Display for LabelError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LabelError::MissingTemplate(template) => write!(f, "[ERROR] LabelCreation - Missing Template - Could not find {}.typ", template),
            LabelError::Compilation() => write!(f, "[ERROR] LabelCreation - Compilation - Failed to compile labels"),
            LabelError::Export() =>  write!(f, "[ERROR] LabelCreation - Export - Failed to export labels")
        }
    }
}

// #[derive(serde::Serialize)]
// struct TransportError{
//     message: String,

// }

impl IntoResponse for LabelError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

