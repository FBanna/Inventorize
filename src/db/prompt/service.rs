
use std::{collections::HashMap, fmt::Debug, ops::Deref, sync::MutexGuard};


use crate::db::{component::component::Component, db::DB};

use super::{prompt::{Prompt, PromptEntry}, prompts::Prompts};



/// new component attributes comes in -> need to add them to the table
/// checks if they exist
///     DO: increment count
///     DONT: add a new row
/// DONE

/// 
pub trait PromptServices {
    
    async fn del_prompt(&self, type_i: i64, index: i32);


    async fn add_prompt(&self, i: usize, value: &str);

    async fn get_prompts(&self, )


    // async fn update_prompt(&self, i: usize, index: i32, count: i32);
    //async fn check_option(&mut self, prompt: &mut Prompt, value: Option<&str>);
    //fn check_prompt_exists(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> bool;
    //fn check_prompt_last(&self, value: &str, mutex: &mut MutexGuard<'_, Vec<PromptEntry>>) -> Option<usize>;
    // async fn update_prompts_add(&self, c: &Component);
    // async fn update_prompts_del(&self, c: &Component);
    // async fn sync_prompts(&self);
}



impl PromptServices for DB{




//     async fn update_prompts_add(&self, c: &Component) {

//         for (i, entry) in c.to_vec().iter().enumerate(){

//             if let Some(some) = *entry{ // if the entry has a value

//                 if !some.is_empty() {

                    
//                     // position , number
//                     let output: Option<(i32, i32)> = {

//                         let mut entries = self.prompt_cache.0[i].prompts.lock().unwrap();


//                         let mut value = None;


//                         for (i, entry) in entries.iter_mut().enumerate(){

//                             if some == entry.0 {

//                                 let int: i32 = i.try_into().unwrap();
//                                 value = Some((int, entry.1.clone() + 1));


//                                 entry.1 += 1;
//                                 //self.update_prompt(i, value)

                                
//                                 break;
//                             }

//                         }

//                         if value.is_none(){
//                             entries.push(PromptEntry(some.to_owned(),1));
//                         }

//                         value
//                     };

//                     if let Some(position_count) = output {
                        
//                         self.update_prompt(i, position_count.0 + 1, position_count.1).await;
//                     } else {
//                         self.add_prompt(i, some).await;
//                     }
                    
                        
//                 }

//             }
//         }
//     }



//     async fn update_prompts_del(&self, c: &Component) {
//         for  (i, entry) in c.to_vec().iter().enumerate() {
//             if let Some(some) = *entry {
//                 if !some.is_empty(){

                    
//                     // delete? , position , number
//                     let delete: Option<(bool, i32,i32)> = {

//                         let mut entries = self.prompt_cache.0[i].prompts.lock().unwrap();

//                         let mut value: Option<(bool, i32,i32)> = None;

//                         for (i, entry) in entries.iter_mut().enumerate(){

//                             if entry.0 == some{
//                                 let int: i32 = i.try_into().unwrap();

//                                 if entry.1 <= 1{
                                    

//                                     value = Some((true, int, 0));
//                                     entries.remove(i);
//                                     break;
//                                 } else {
//                                     value = Some((false, int, (entry.1.clone() - 1)));
//                                     entry.1 -= 1;
                                    
//                                 }

//                             }

//                         }

//                         value
//                     };

//                     if let Some(value) = delete {

//                         if !value.0 {
//                             self.update_prompt(i, value.1 + 1, value.2).await;
//                         } else {
//                             self.del_prompt(i, value.1 + 1).await;
//                         }
                        
//                     }

//                 }
//             }
//         }
//     }


    

//     async fn add_prompt(&self, i: usize, value: &str) {

//         //println!("EDIT TO DB: ADD_PROMPT - prompt cache = {}, value = {}", self.prompt_cache.0[i].name, value);

        

//         let prompt = &self.prompt_cache.0[i];

//         let string = "INSERT INTO ".to_owned() + &prompt.name + " (entry,n) VALUES (?,?)";
        
//         sqlx::query(&string)
//             .bind(value)
//             .bind(1)
//             .execute(&*self.pool)
//             .await
//             .unwrap();
//     }

//     async fn del_prompt(&self, i: usize, index: i32) {

//         //println!("EDIT TO DB: DEL_PROMPT - prompt cache = {}, position = {}", self.prompt_cache.0[i].name, index);

//         let prompt = &self.prompt_cache.0[i];

//         let string = "DELETE FROM ".to_owned() + &prompt.name + " WHERE ROWID = (?)";

//         sqlx::query(&string)
//             .bind(index)
//             .execute(&*self.pool)
//             .await
//             .unwrap();
//     }

//     async fn update_prompt(&self, i: usize, index: i32, count: i32) {

//         //println!("EDIT TO DB: UPDATE_PROMPT - prompt cache = {}, position = {}, count = {}", self.prompt_cache.0[i].name, index, count);

//         let prompt = &self.prompt_cache.0[i];

//         //UPDATE name SET n = 3 WHERE ROWID = 2

//         let string = "UPDATE ".to_owned() + &prompt.name + " SET n = (?) WHERE ROWID = (?)";

//         sqlx::query(&string)
//             .bind(count)
//             .bind(index)
//             .execute(&*self.pool)
//             .await
//             .unwrap();

//     }
    
//     async fn sync_prompts(&self) {

        

//         for i in 0..self.prompt_cache.0.len(){


            

//             let string = "SELECT * FROM ".to_owned() + &self.prompt_cache.0[i].name;

//             // let temp: HashMap<String, i32> = sqlx::query_as(&string) POTENTIALLY BENEFICIAL TO REWRITE IN ANOTHER DATA FORMAT
                
//             //     .fetch_all(&self.pool)
//             //     .await
//             //     .unwrap();



            
//             let result: Vec<PromptEntry> = sqlx::query_as(&string)
                
//                 .fetch_all(&*self.pool)
//                 .await
//                 .unwrap();

//             let mut entries_mutex = self.prompt_cache.0[i].prompts.lock().unwrap();

//             *entries_mutex = result;

//             //self.prompt_cache.0[i].prompts = result;    
//         }
//     }

    
}



