use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum LabelErrors {

    MissingTemplate(String),
    Compilation,
    Export

}


impl std::error::Error for LabelErrors {}



impl Display for LabelErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LabelErrors::MissingTemplate(template) => write!(f, "[ERROR] LabelCreation - Missing Template - Could not find {}.typ", template),
            LabelErrors::Compilation => write!(f, "[ERROR] LabelCreation - Compilation - Failed to compile labels"),
            LabelErrors::Export =>  write!(f, "[ERROR] LabelCreation - Export - Failed to export labels")
        }
    }
}

// #[derive(serde::Serialize)]
// struct TransportError{
//     message: String,

// }

impl IntoResponse for LabelErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

