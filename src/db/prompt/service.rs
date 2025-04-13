
use crate::db::{components::Component, db::DB};

use super::prompt::{Prompt, PromptEntry};



pub trait PromptServices {
    async fn update_prompt(&self, prompt: String, value: String);
    async fn add_prompt(&mut self, prompt: String, value: String);
    async fn check_option(&self, prompt: String, value: Option<String>);
    async fn check_prompt_exists(&self, prompt: String, value: String) -> bool;
    async fn update_prompts(&self, c: Component);
    async fn sync_prompts(&mut self);
}



impl PromptServices for DB{

    async fn check_option(&self, prompt: String, value: Option<String>){
    
        if let Some(some) = value{
            self.check_prompt_exists(prompt, some);
        }
    }
    
    async fn update_prompts(&self, c: Component) {

        let prompts = &self.prompt_cache.0;
        self.check_prompt_exists(prompts[0].name.clone(), c.name);
        self.check_option(prompts[1].name.clone(), c.size).await;
        self.check_option(prompts[2].name.clone(), c.value).await;
        self.check_option(prompts[3].name.clone(), c.info).await;
        self.check_option(prompts[4].name.clone(), c.origin).await;
        self.check_option(prompts[5].name.clone(), c.label).await;
    }

    async fn update_prompt(&self, prompt: String, value: String){
        if !self.check_prompt_exists(prompt.clone(), value.clone()).await {
            self.add_prompt(prompt, value).await
        }
    }

    async fn check_prompt_exists(&self, prompt: String, value: String) -> bool {
        
        let i: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM (?) where entry = (?)")
            .bind(prompt)
            .bind(value)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        if i > 0{
            return false;
        } else {
            return true;
        }

    }

    async fn add_prompt(&mut self, prompt: String, value: String) {
        sqlx::query("INSERT INTO (?) (entry) VALUES (?)")
            .bind(prompt)
            .bind(value)
            .execute(&self.pool)
            .await
            .unwrap();
    }
    
    async fn sync_prompts(&mut self) {

        for i in 0..self.prompt_cache.0.len(){
            let result: Vec<PromptEntry> = sqlx::query_as("SELECT * FROM (?)")
            .bind(self.prompt_cache.0[i].name.clone())
            .fetch_all(&self.pool)
            .await
            .unwrap();

            self.prompt_cache.0[i].prompts = result;    
        }
    }

    
}






// pub trait PromptServices{
//     async fn get_prompt(&self, col: &str) -> Vec<Prompt>;
//     async fn get_prompts(&self) -> Prompts;
//     async fn update_prompts(&self, c: Component);
//     async fn update_prompt(&self, table: &str, entry: &str);
//     async fn update_optional_prompt(&self, table: &str, entry: Option<String>);
//     async fn check_exists(&self, table: &str, entry: &str) -> bool;
//     async fn add_prompts(&self, prompts: Prompts);
//     async fn add_prompt(&self, prompt: Prompt, col: &str);
// }



// impl PromptServices for DB {

//     async fn update_optional_prompt(&self, table: &str, entry: Option<String>) {
//         if let Some(some) = entry{
//             self.update_prompt(table, &some).await;
//         }
//     }

//     async fn update_prompt(&self, table: &str, entry: &str) {
//         let bool: bool = self.check_exists(table, entry).await;
//         if !bool {
//             self.add_prompt(Prompt(entry.to_owned()), table).await;
//         }
//     }

//     async fn check_exists(&self, table: &str, entry: &str) -> bool {
//         let i: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM (?) where entry = (?)")
//             .bind(table)
//             .bind(entry)
//             .fetch_one(&self.pool)
//             .await
//             .unwrap();

//         if i > 0{
//             return false;
//         } else {
//             return true;
//         }
//     }



//     async fn update_prompts(&self, c: Component) {
//         self.update_prompt("name", &c.name).await;
//         self.update_optional_prompt("size", c.size).await;
//         self.update_optional_prompt("value", c.value).await;
//         self.update_optional_prompt("info", c.info).await;
//         self.update_optional_prompt("origin", c.origin).await;
//         self.update_optional_prompt("label", c.label).await;
//     }

//     async fn add_prompts(&self, prompts: Prompts) {
        
//         let tables = ["name", "size", "value", "info", "origin", "label"];
        
//         for table in tables {

//         }
//     }
    
//     async fn add_prompt(&self, prompt: Prompt, col: &str) {
        
//     }

//     async fn get_prompt(&self, col: &str) -> Vec<Prompt> {
//         println!("This is it = {}", col);
//         let result: Vec<Prompt> = sqlx::query_as("SELECT * FROM (?)")
//             .bind(col)
//             .fetch_all(&self.pool)
//             .await
//             .unwrap();

//         return result;
//     }
    
//     async fn get_prompts(&self) -> Prompts{

//         Prompts {
//             name: self.get_prompt("name").await,
//             size: self.get_prompt("size").await,
//             value: self.get_prompt("value").await,
//             info: self.get_prompt("info").await,
//             origin: self.get_prompt("origin").await,
//             label: self.get_prompt("label").await,
//         }
//     }
    
    
// }

