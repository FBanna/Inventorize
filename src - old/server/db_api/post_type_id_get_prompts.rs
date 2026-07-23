use std::{sync::Arc};

use axum::{extract::State};
use axum_extra::response::ErasedJson;

use crate::server::server_state::ServerState;


/// takes in a type_id -> fetches all distinct values of all attributes
/// returns it as a json object 
/// {
///     resistance: [100, 20, 60, 80, 110, 111, ...],
///     package: ["0402", "0603", ...]
/// }
pub async fn post_type_id_get_prompts(
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