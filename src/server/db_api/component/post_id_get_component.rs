use std::{result, sync::Arc};

use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::component::{component::Component, service::ComponentServices}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    component_id: Uuid
}

pub async fn post_id_get_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<Json<Component>, AppError> {


    let result = shared_state.db.get_component(component.component_id).await?;

    Ok(Json(result))
}