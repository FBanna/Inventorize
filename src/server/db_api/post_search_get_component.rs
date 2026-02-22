use std::sync::Arc;

use axum::{extract::State, Json};
use crate::{db::{components::{Component, ComponentServices}}, error::error::AppError,  server::server_state::ServerState};



pub async fn post_search_get_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<Vec<Vec<String>>>
) -> Result<Json<Vec<Component>>, AppError> {

    let result = shared_state.db.search(c).await?;

    Ok(Json(result))
}