use serde::Serialize;

use crate::db::components::ELEMENTS;

use super::prompt::Prompt;


#[derive(Debug, Serialize)]
pub struct Prompts(pub Vec<Prompt>);



impl Prompts{
    pub fn new() -> Self {
        let prompt_names = ELEMENTS;

        let mut prompts = Vec::with_capacity(ELEMENTS.len());

        for prompt in prompt_names {
            prompts.push(Prompt::new(prompt.to_owned()));
        }

        return Prompts(prompts);
    }

    // pub fn sync(&self, pool: Pool<Sqlite>) {
        
    //     for prompt in &self.0{
    //         prompt.sync();
    //     }

    // }
}