use serde::Serialize;

use super::prompt::Prompt;


#[derive(Debug, Serialize)]
pub struct Prompts(pub Vec<Prompt>);



impl Prompts{
    pub fn new() -> Self {
        let prompt_names = ["namep","sizep","valuep","infop","originp","labelp"];

        let mut prompts = Vec::with_capacity(6);

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