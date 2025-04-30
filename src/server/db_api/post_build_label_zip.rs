use std::sync::Arc;

use axum::{body::Bytes, extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct BuildLabelZip{
    list: Vec<i32>
}

pub async fn post_build_label(

    State(shared_state): State<Arc<ServerState>>,
    Json(components): Json<BuildLabelZip>
    
) -> impl IntoResponse {

    let option = shared_state.db.get_from_list(components.list).await;

    Component::build_zip(option, &shared_state.config);

    //let option = shared_state.db.get(component.i).await.build(&shared_state.config);

    if let Some(bytes) = option {

        let array = Bytes::from_owner(bytes);

        return array.into_response();
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }

}