
// use std::{error, fmt::{format, Display}};

// use axum::{http::StatusCode, response::IntoResponse};



// #[derive(Debug)]
// pub struct DBError {
//     pub message: String
// }

// impl std::error::Error for DBError {}

// impl Display for DBError{
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//         write!(f, "[ERROR] DB Error -{}", self.message)
//     }
// }

// impl From<sqlx::Error> for DBError {
//     fn from(value: sqlx::Error) -> Self {
//         DBError { message: value.to_string() }
//     }
// }

// impl IntoResponse for DBError{
//     fn into_response(self) -> axum::response::Response {
//         (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
//     }
// }