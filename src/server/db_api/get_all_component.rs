use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use crate::{db::components::{Component, ComponentServices}, error::db::DBError, server::server_state::ServerState};


pub async fn get_component(
    State(shared_state): State<Arc<ServerState>>,
) -> Json<Vec<Component>>{

    let result = shared_state.db.get_all().await;

    match result{
        Ok(list) => Json(list),
        Err(E) => {
            DBError::from(E).into_response()
        }
    }

    //Json(result)
}

