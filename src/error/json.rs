use std::{error, fmt::{format, Display}};

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum JsonError {

    TypeAttributeEvaluation(String),
    GenSchema,

}


impl std::error::Error for JsonError {}



impl Display for JsonError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsonError::TypeAttributeEvaluation(error) => write!(f, "[ERROR] TypeAttributeEvaluation - Could not evaluate attributes for component: {}", error),
            JsonError::GenSchema => write!(f, "[ERROR] GenSchema - Could not generate schema"),
        }
    }
}


impl IntoResponse for JsonError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

