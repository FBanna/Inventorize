use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};



pub async fn post_search_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<Component>
) -> Json<Vec<Component>> {

    let result = shared_state.db.search(c).await;

    let test = &shared_state.db.prompt_cache;

    println!("{:?}", test);

    return Json(result);
}