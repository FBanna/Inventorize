use std::{fmt::Debug, sync::Arc};

use axum::{extract::State, Json};
use axum_extra::response::ErasedJson;
use typst::syntax::ast::RenamedImportItem;
use crate::{db::{components::{Component, ComponentServices}, prompt::{prompt::{Prompt, PromptEntry}, prompts::Prompts}}, server::server_state::ServerState};

// THIS IS NOT NICE FIX THIS

pub async fn get_all_prompt(
    State(shared_state): State<Arc<ServerState>>,
) -> ErasedJson{

    // let prompts = shared_state.db.prompt_cache;


    // let json = Json(prompts);

    //return Json(shared_state.db.prompt_cache);

    return ErasedJson::new(&shared_state.db.prompt_cache.0);

        // let mut temp = Vec::new();

    // for prompt in prompts{
    //     temp.push(
    //         (
    //             prompt.name.clone(),
    //             prompt.prompts.lock().unwrap().as_slice().to_vec()
    //         )
    //     );
    // }

}