use std::{result, sync::Arc};

use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::component::{component::{Component, ComponentWithAttributes}, service::ComponentServices}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    component_id: Uuid
}

pub async fn post_id_get_component_with_attributes(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<Json<ComponentWithAttributes>, AppError> {


    let result: crate::db::component::component::ComponentWithAttributes = shared_state.db.get_component_with_attributes(component.component_id).await?;

    Ok(Json(result))
}