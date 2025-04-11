use std::path::Components;

use clap::builder::Str;
use sqlx::prelude::FromRow;

use super::{components::Component, db::DB};

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
    async fn update_prompts(&self, c: Component);
    async fn update_prompt(&self, table: &str, entry: &str);
    async fn update_optional_prompt(&self, table: &str, entry: Option<String>);
    async fn check_exists(&self, table: &str, entry: &str) -> bool;
    async fn add_prompts(&self, prompts: Prompts);
    async fn add_prompt(&self, prompt: Prompt, col: &str);
}



impl PromptServices for DB {

    async fn update_optional_prompt(&self, table: &str, entry: Option<String>) {
        if let Some(some) = entry{
            self.update_prompt(table, &some).await;
        }
    }

    async fn update_prompt(&self, table: &str, entry: &str) {
        let bool: bool = self.check_exists(table, entry).await;
        if !bool {
            self.add_prompt(Prompt(entry.to_owned()), table).await;
        }
    }

    async fn check_exists(&self, table: &str, entry: &str) -> bool {
        let i: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM (?) where entry = (?)")
            .bind(table)
            .bind(entry)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        if i > 0{
            return false;
        } else {
            return true;
        }
    }



    async fn update_prompts(&self, c: Component) {
        self.update_prompt("name", &c.name).await;
        self.update_optional_prompt("size", c.size).await;
        self.update_optional_prompt("value", c.value).await;
        self.update_optional_prompt("info", c.info).await;
        self.update_optional_prompt("origin", c.origin).await;
        self.update_optional_prompt("label", c.label).await;
    }

    async fn add_prompts(&self, prompts: Prompts) {
        
        let tables = ["name", "size", "value", "info", "origin", "label"];
        
        for table in tables {

        }
    }
    
    async fn add_prompt(&self, prompt: Prompt, col: &str) {
        
    }

    async fn get_prompt(&self, col: &str) -> Vec<Prompt> {
        println!("This is it = {}", col);
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

