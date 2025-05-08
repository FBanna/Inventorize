use std::sync::Arc;

use axum::{extract::State, http::{Response, StatusCode}, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    i: i32
}

pub async fn post_id_remove_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> impl IntoResponse {


    
    //shared_state.db.get_first().await.build(&shared_state.config.label_location);

    shared_state.db.remove(component.i).await;

    return StatusCode::OK.into_response();
}