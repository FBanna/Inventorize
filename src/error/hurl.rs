use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum HurlErrors {

    MissingFile(String),
    NoFile,
    Run(String),
    NoField(String),
    ImproperField(String)

}


impl std::error::Error for HurlErrors {}



impl Display for HurlErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //ClassError::ExpectedAttributes => write!(f, "[ERROR] ClassError - ExpectedAttributes - Expected type to contain attributes")
            HurlErrors::MissingFile(template) => write!(f, "[ERROR] HurlError - MissingFile - Could not find path to hurl file: {}", template),
            HurlErrors::NoFile => write!(f, "[ERROR] HurlError - NoFile - Origin does contain that file!"),
            HurlErrors::Run(template) => write!(f, "[ERROR] HurlError - Run - Failed to run hurl file with error: {}", template),
            HurlErrors::NoField(template) => write!(f, "[ERROR] HurlError - NoField - Hurl did not respond with field: {}", template),
            HurlErrors::ImproperField(template) => write!(f, "[ERROR] HurlError - ImproperField - Hurl did not respond with the correct field: {}", template)

        }
    }
}


impl IntoResponse for HurlErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

