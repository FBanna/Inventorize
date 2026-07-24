use std::{error, fmt::{Display, format}, write};

use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum JsonErrors {

    ClassAttributesMalformed(String),
    GenSchema,
    ComponentClassAttributesMalformed(String),
    GenValidator

}


impl std::error::Error for JsonErrors {}



impl Display for JsonErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsonErrors::ClassAttributesMalformed(error) => write!(f, "[ERROR] ClassAttributesMalformed - Could not evaluate attributes for component type: {}", error),
            JsonErrors::GenSchema => write!(f, "[ERROR] GenSchema - Could not generate schema"),
            JsonErrors::ComponentClassAttributesMalformed(error) => write!(f, "[ERROR] ComponentClassAttributesMalformed - Could not evaluate attributes for component: {}", error),
            JsonErrors::GenValidator => write!(f, "[ERROR] GenValidator - Could not generate validator from schema"),
        }
    }
}


impl IntoResponse for JsonErrors{
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

