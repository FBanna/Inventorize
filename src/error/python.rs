use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum PythonErrors {

    
    MissingFile(String),
    NoFile,
    Run(String),
    PathError
    // NoField(String),
    // ImproperField(String)

}


impl std::error::Error for PythonErrors {}



impl Display for PythonErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            //ClassError::ExpectedAttributes => write!(f, "[ERROR] ClassError - ExpectedAttributes - Expected type to contain attributes")
            PythonErrors::MissingFile(template) => write!(f, "[ERROR] PythonError - MissingFile - Could not find path to file: {}", template),
            PythonErrors::NoFile => write!(f, "[ERROR] PythonError - NoFile - Origin does contain that file!"),
            PythonErrors::Run(template) => write!(f, "[ERROR] PythonError - Run - Failed to run file with error: {}", template),
            PythonErrors::PathError => write!(f, "ERROR PythonError - PathError - Error in path to python file!")
            // PythonErrors::NoField(template) => write!(f, "[ERROR] PythonError - NoField - Hurl did not respond with field: {}", template),
            // PythonErrors::ImproperField(template) => write!(f, "[ERROR] PythonError - ImproperField - Hurl did not respond with the correct field: {}", template)

        }
    }
}


impl IntoResponse for PythonErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

