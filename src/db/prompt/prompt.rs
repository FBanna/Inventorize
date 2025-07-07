use std::{path::Components, sync::Mutex};

use clap::builder::Str;
use serde::{ser::SerializeStruct, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use super::super::{components::Component, db::DB};


#[derive(FromRow,Debug,Serialize)]
pub struct PromptEntry( pub String, pub i32);

#[derive(Debug)]
pub struct Prompt{
    pub name: String,
    pub prompts: Mutex<Vec<PromptEntry>>
}

impl Serialize for Prompt{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        

        let mut state = serializer.serialize_struct("Prompt",2)?;
        state.serialize_field("name", &self.name)?;

        let prompts = self.prompts.lock().unwrap();
        state.serialize_field("prompts", &*prompts)?;
        state.end()
    }
}

impl Prompt{
    pub fn new(name: String) -> Self {
        return Prompt{name, prompts: Mutex::new(Vec::new())};
    }

    pub fn add(&mut self, entry: PromptEntry) {
        self.prompts.lock().unwrap().push(entry);
    }

}


