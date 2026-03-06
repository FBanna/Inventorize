use std::{fmt::Display, sync::Arc};

use axum::{http::{StatusCode, response}, response::{IntoResponse, Response}};

use crate::error::{json::JsonError, label::LabelError};

// helped greatly by - https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs

#[derive(Debug)]
pub enum AppError{

    DBError(sqlx::Error),

    LabelError(LabelError),

    JsonError(JsonError)

}


impl std::error::Error for AppError{}

impl Display for AppError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::DBError(err) => write!(f, "[ERROR] DB Error - {}", err.to_string()),
            AppError::LabelError(err) => err.fmt(f),
            _ => write!(f, "[ERROR] Unknown Error")
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let ( mut response, err_option) = match &self {

            AppError::LabelError(err) => {
                (err.clone().into_response(), Some(self))
            },
            AppError::JsonError(err) => {
                (err.clone().into_response(), Some(self))
            },
            AppError::DBError(err) => {

                (((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()), Some(self))

            },
            _ => ((StatusCode::INTERNAL_SERVER_ERROR, "Unknown Inventorize error!").into_response(), None)
        };

        if let Some(err) = err_option {
            response.extensions_mut().insert(Arc::new(err));
        }

        response


    }
}



// INTERNAL


impl From<LabelError> for AppError {
    fn from(value: LabelError) -> Self {
        Self::LabelError(value)
    }
}

impl From<JsonError> for AppError {
    fn from(value: JsonError) -> Self {
        Self::JsonError(value)
    }
}


// EXTERNAL

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value)
    }
}

