use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum ClassErrors {

    AttributeParsing(String)

}


impl std::error::Error for ClassErrors {}



impl Display for ClassErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //ClassError::ExpectedAttributes => write!(f, "[ERROR] ClassError - ExpectedAttributes - Expected type to contain attributes")
            ClassErrors::AttributeParsing(template) => write!(f, "[ERROR] ClassError - AttributeParsing - Failed to parse class attributes: {}", template)
        }
    }
}


impl IntoResponse for ClassErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

