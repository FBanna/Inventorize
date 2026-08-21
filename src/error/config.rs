use std::{error, fmt::{format, Display}};

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, Clone)]
pub enum ConfigErrors {

    MissingConfig,
    Deserialisation

}


impl std::error::Error for ConfigErrors {}



impl Display for ConfigErrors{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigErrors::MissingConfig => write!(f, "[ERROR] Config - Missing Config"),
            ConfigErrors::Deserialisation => write!(f, "[ERROR] Config - Failed to Deserialise")
        }
    }
}
