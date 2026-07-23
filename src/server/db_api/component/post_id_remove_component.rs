use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::component::service::ComponentServices, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    component_id: Uuid
}

pub async fn post_id_remove_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<impl IntoResponse, AppError> {

    shared_state.db.remove_component(component.component_id, &shared_state.config).await?;

    Ok(StatusCode::OK.into_response())

    
}