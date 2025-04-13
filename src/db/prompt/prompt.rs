use std::path::Components;

use clap::builder::Str;
use sqlx::{prelude::FromRow, Pool, Sqlite};

use super::super::{components::Component, db::DB};


#[derive(FromRow,Debug)]
pub struct PromptEntry(String);

#[derive(Debug)]
pub struct Prompt{
    pub name: String,
    pub prompts: Vec<PromptEntry>
}

impl Prompt{
    pub fn new(name: String) -> Self {
        return Prompt{name, prompts: Vec::new()};
    }

}


