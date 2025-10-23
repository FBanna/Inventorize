use std::sync::Arc;

use axum::{body::Bytes, extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct BuildLabelZip{
    list: Vec<i32>
}

pub async fn post_build_label_zip(

    State(shared_state): State<Arc<ServerState>>,
    Json(components): Json<BuildLabelZip>
    
) -> impl IntoResponse {

    println!("1: starting");

    let list = shared_state.db.get_from_list(components.list).await;

    println!("2");

    let option = Component::build_pdf(list, &shared_state.config);

    println!("3");
    //let option = shared_state.db.get(component.i).await.build(&shared_state.config);

    if let Some(bytes) = option {

        println!("4");

        let array = Bytes::from_owner(bytes);

        println!("5: finished");

        return array.into_response();

        
    } else {
        return StatusCode::NOT_FOUND.into_response();
    }

}