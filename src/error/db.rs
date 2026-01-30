
use std::{error, fmt::{format, Display}};



#[derive(Debug)]
pub enum DBError {
    QueryFail(String)
}

impl std::error::Error for DBError {}

impl Display for DBError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DBError::QueryFail(template) => write!(f, "[ERROR] DB Query - {}", template)
        }
    }
}