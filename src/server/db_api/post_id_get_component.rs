use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    i: i32
}

pub async fn post_id_get_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Json<Component> {


    
    //shared_state.db.get_first().await.build(&shared_state.config.label_location);

    return Json(shared_state.db.get(component.i,&shared_state.config).await);
}