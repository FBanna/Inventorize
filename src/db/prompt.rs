use std::path::Components;

use sqlx::prelude::FromRow;

use super::db::DB;

#[derive(FromRow, Debug)]
pub struct Prompt(String);


#[derive(Debug)]
pub struct Prompts {
    pub name: Vec<Prompt>,
    pub size: Vec<Prompt>,
    pub value: Vec<Prompt>,
    pub info: Vec<Prompt>,
    pub origin: Vec<Prompt>,
    pub label: Vec<Prompt>,
}

pub trait PromptServices{
    async fn get_prompt(&self, col: &str) -> Vec<Prompt>;
    async fn get_prompts(&self) -> Prompts;
}

impl PromptServices for DB {

    async fn get_prompt(&self, col: &str) -> Vec<Prompt> {
        let result: Vec<Prompt> = sqlx::query_as("SELECT * FROM (?)")
            .bind(col)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        return result;
    }
    
        async fn get_prompts(&self) -> Prompts{

        Prompts {
            name: self.get_prompt("name").await,
            size: self.get_prompt("size").await,
            value: self.get_prompt("value").await,
            info: self.get_prompt("info").await,
            origin: self.get_prompt("origin").await,
            label: self.get_prompt("label").await,
        }
    }
}

