use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::component::service::ComponentServices, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    component_ids: Vec<Uuid>
}

pub async fn post_id_remove_list_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<impl IntoResponse, AppError> {

    shared_state.db.remove_component_list(component.component_ids, &shared_state.config).await?;

    Ok(StatusCode::OK.into_response())
}