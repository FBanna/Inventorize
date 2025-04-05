use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use crate::{cli::config::Config, db::{self, components::{Component, Components}}, label::label::Label, server::server_state::ServerState};


pub async fn post_build_label(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<i32>
) -> impl IntoResponse {

    println!("IM HERE!!");

    shared_state.db.get_first().await.build(&shared_state.config.label_location);

    return StatusCode::OK.into_response();
}