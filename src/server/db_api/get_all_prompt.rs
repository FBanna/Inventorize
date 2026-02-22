use std::{sync::Arc};

use axum::{extract::State};
use axum_extra::response::ErasedJson;

use crate::server::server_state::ServerState;



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