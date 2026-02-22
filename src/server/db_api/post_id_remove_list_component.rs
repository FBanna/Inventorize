use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{db::components::ComponentServices, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    i: Vec<i32>
}

pub async fn post_id_remove_list_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<impl IntoResponse, AppError> {

    shared_state.db.remove_list(component.i).await?;

    Ok(StatusCode::OK.into_response())
}