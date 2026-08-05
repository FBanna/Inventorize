use std::{fmt::Display, io, sync::Arc};

use axum::{extract::multipart::MultipartError, http::{StatusCode, response}, response::{IntoResponse, Response}};
use jsonschema::ValidationError;
use serde::de::value;

use crate::error::{file::{FileErrors}, json::JsonErrors, label::LabelErrors, class::ClassErrors};

// helped greatly by - https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs

#[derive(Debug)]
pub enum AppError{

    DBError(sqlx::Error),

    LabelError(LabelErrors),

    JsonError(JsonErrors),

    TypeError(ClassErrors),

    FileError(FileErrors)

}


impl std::error::Error for AppError{}

impl Display for AppError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::DBError(err) => {

                let db_err_op = err.as_database_error();

                if let Some(db_err) = db_err_op {

                    return write!(f, "[ERROR] DB Error - {}", db_err.message());
                }

                write!(f, "[ERROR] DB Error - {}", err)
            },
            AppError::LabelError(err) => err.fmt(f),
            AppError::JsonError(err) => err.fmt(f),
            AppError::TypeError(err) => err.fmt(f),
            AppError::FileError(err) => err.fmt(f),
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
            AppError::FileError(err) => {
                (err.clone().into_response(), Some(self))
            },
            AppError::DBError(err) => {

                (((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()), Some(self))

            },
            AppError::TypeError(err) => {
                (err.clone().into_response(), Some(self))
            }
            _ => ((StatusCode::INTERNAL_SERVER_ERROR, "Unknown Inventorize error!").into_response(), None)
        };

        if let Some(err) = err_option {
            response.extensions_mut().insert(Arc::new(err));
        }

        response


    }
}



// INTERNAL

// fn from(e: std::io::Error) -> Self {
//         MyError::Io(e)
//     }



impl From<LabelErrors> for AppError {
    fn from(value: LabelErrors) -> Self {
        Self::LabelError(value)
    }
}

impl From<JsonErrors> for AppError {
    fn from(value: JsonErrors) -> Self {
        Self::JsonError(value)
    }
}

impl From<ClassErrors> for AppError {
    fn from(value: ClassErrors) -> Self {
        Self::TypeError(value)
    }
}

impl From<FileErrors> for AppError {
    fn from(value: FileErrors) -> Self {
        Self::FileError(value)
    }
}


// EXTERNAL

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(JsonErrors::GenSchema)
    }
}

impl From<MultipartError> for AppError {
    fn from(value: MultipartError) -> Self {
        Self::FileError(FileErrors::Upload(value.body_text()))
    }
}


impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::FileError(FileErrors::WriteUpload)
    }
}

impl<'a> From<ValidationError<'a>> for AppError {
    fn from(value: ValidationError) -> Self {
        Self::JsonError(JsonErrors::GenValidator)
    }
}