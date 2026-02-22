use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{config::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};



pub async fn post_search_get_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<Vec<Vec<String>>>
) -> Json<Vec<Component>> {

    let result = shared_state.db.search(c).await;

    return Json(result);
}