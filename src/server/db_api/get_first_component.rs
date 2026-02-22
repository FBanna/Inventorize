use std::sync::Arc;

use axum::{extract::State, Json};
use crate::{db::components::{Component, ComponentServices}, error::error::AppError, server::server_state::ServerState};


pub async fn get_component(
    State(shared_state): State<Arc<ServerState>>,
) -> Result<Json<Component>,AppError>{

    let result = shared_state.db.get_first().await?;

    Ok(Json(result))
}