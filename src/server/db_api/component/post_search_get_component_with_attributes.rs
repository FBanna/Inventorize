use std::{sync::Arc, todo};

use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;
use crate::{db::{component::{component::{Component, ComponentWithAttributes}, service::ComponentServices}, component_class::{component_class::ComponentClassSearch, service::ComponentClassServices}}, error::error::AppError, server::server_state::ServerState};




pub async fn post_search_get_component_with_attributes(

    State(shared_state): State<Arc<ServerState>>,
    Json(q): Json<Vec<ComponentClassSearch>>
) -> Result<Json<Vec<ComponentWithAttributes>>, AppError> {

    //let result = shared_state.db.search(c).await?;

    let result = shared_state.db.search_components_on_component_class(q).await?;

    Ok(Json(result))

}