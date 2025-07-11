
use std::{ops::Deref, sync::MutexGuard};

use crate::db::{components::Component, db::DB};

use super::{prompt::{Prompt, PromptEntry}, prompts::Prompts};



pub trait PromptServices {
    async fn del_prompt(&self, i: usize, index: i32);
    //async fn update_prompt(&self, prompt: String, value: String);
    async fn add_prompt(&self, i: usize, value: &str);
    //async fn check_option(&mut self, prompt: &mut Prompt, value: Option<&str>);
    fn check_prompt_exists(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> bool;
    fn check_prompt_last(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> Option<usize>;
    async fn update_prompts_add(&self, c: &Component);
    async fn update_prompts_del(&self, c: &Component);
    async fn sync_prompts(&self);
}



impl PromptServices for DB{

    // async fn check_option(&mut self, prompt: &mut Prompt, value: Option<&str>){
    
    //     if let Some(some) = value{
    //         if !self.check_prompt_exists(prompt, some).await {
    //             self.add_prompt(prompt, some);
    //         }
    //     }
    // }

    fn check_prompt_exists(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> bool {

        // ADD FAST ORDERED SEARCH HERE

        for entry in mutex.iter_mut() {

            if value == &entry.0 {
                entry.1 += 1; // INCREMANTS COUNT IF FOUND
                return true;
            }
        }

        return false;
        
        
        // let i: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM (?) where entry = (?)")
        //     .bind(prompt)
        //     .bind(value)
        //     .fetch_one(&self.pool)
        //     .await
        //     .unwrap();

        // if i > 0{
        //     return false;
        // } else {
        //     return true;
        // }

    }


    // RETURNS -1 if not last and the index of the value if it is the last
    fn check_prompt_last(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> Option<usize> {
        
        for (i, entry) in mutex.iter_mut().enumerate() {

            if value == &entry.0 {

                if entry.1 == 1 {
                    return Some(i);
                } else if entry.1 > 1 {
                    entry.1 -= 1;
                    return None;
                }

            }

        }

        println!("Deleting a prompt that does not exist!");
        return None;

        
    }
    
    async fn update_prompts_add(&self, c: &Component) {

        for (i, entry) in c.to_vec().iter().enumerate(){

            if let Some(some) = *entry{ // if the entyr has a value

                if !some.is_empty() {

                    let exists = {
                        let mut entries = self.prompt_cache.0[i].prompts.lock().unwrap();
                        let bool = self.check_prompt_exists(some, &mut entries);

                        if !bool {
                            entries.push(PromptEntry(some.to_owned(),1));
                        }

                        bool
                    };

                    if !exists{

                        self.add_prompt(i, some).await;
                    }
                    
                        
                        
                }

                
        
            }
        }
    }

    async fn update_prompts_del(&self, c: &Component) {
        for  (i, entry) in c.to_vec().iter().enumerate() {
            if let Some(some) = *entry {
                if !some.is_empty(){

                    let (bool, value): (bool, i32) = {
                        let mut entries = self.prompt_cache.0[i].prompts.lock().unwrap();
                        let option = self.check_prompt_last(some, &mut entries);

                        

                        if let Some(index) = option {
                            let int: i32 = index.try_into().unwrap();
                            entries.remove(index);

                            

                            (true, int)


                        } else {
                            (false,0)
                        }

                        
                    };

                    if bool {
                        self.del_prompt(i, value).await;
                    }
                }
            }
        }
    }

    // async fn update_prompt(&self, prompt: String, value: String){
    //     if !self.check_prompt_exists(prompt.clone(), value.clone()).await {
    //         self.add_prompt(prompt, value).await
    //     }
    // }

    

    async fn add_prompt(&self, i: usize, value: &str) {

        

        let prompt = &self.prompt_cache.0[i];

        let string = "INSERT INTO ".to_owned() + &prompt.name + " (entry,n) VALUES (?,?)";
        
        sqlx::query(&string)
            .bind(value)
            .bind(1)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    async fn del_prompt(&self, i: usize, index: i32) {

        let prompt = &self.prompt_cache.0[i];

        let string = "DELETE FROM ".to_owned() + &prompt.name + " WHERE ROWID = (?)";

        sqlx::query(&string)
            .bind(index)
            .execute(&self.pool)
            .await
            .unwrap();
    }
    
    async fn sync_prompts(&self) {

        

        for i in 0..self.prompt_cache.0.len(){


            

            let string = "SELECT * FROM ".to_owned() + &self.prompt_cache.0[i].name;
            
            let result: Vec<PromptEntry> = sqlx::query_as(&string)
                
                .fetch_all(&self.pool)
                .await
                .unwrap();

            let mut entries_mutex = self.prompt_cache.0[i].prompts.lock().unwrap();

            *entries_mutex = result;

            //self.prompt_cache.0[i].prompts = result;    
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

