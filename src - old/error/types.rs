use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum TypeError {

    ExpectedAttributes

}


impl std::error::Error for TypeError {}



impl Display for TypeError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TypeError::ExpectedAttributes => write!(f, "[ERROR] TypeError - ExpectedAttributes - Expected type to contain attributes")
        }
    }
}


impl IntoResponse for TypeError{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

