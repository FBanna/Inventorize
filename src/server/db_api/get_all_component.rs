use std::sync::Arc;

use axum::{extract::State, Json};
use crate::{db::components::{Component, ComponentServices}, server::server_state::ServerState};


pub async fn get_component(
    State(shared_state): State<Arc<ServerState>>,
) -> Json<Vec<Component>>{

    let result = shared_state.db.get_all(&shared_state.config).await;

    Json(result)
}