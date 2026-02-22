use std::sync::Arc;

use axum::{Json, extract::State};
use crate::{db::components::{Component, ComponentServices}, error::{error::AppError}, server::server_state::ServerState};


pub async fn get_component(
    State(shared_state): State<Arc<ServerState>>,
) -> Result<Json<Vec<Component>>, AppError>{

    let result = shared_state.db.get_all().await?;

    Ok(Json(result))

}

