use std::sync::Arc;

use axum::{body::Bytes, extract::State, http::StatusCode, response::{IntoResponse, Redirect}, Form, Json};
use serde::Deserialize;
use crate::{cli::config::Config, db::{self, components::{Component, ComponentServices}}, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct BuildLabel{
    i: i32
}

pub async fn post_build_label(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<BuildLabel>
) -> Bytes {

    //shared_state.db.get(component.i).await.build_save(&shared_state.config.label_location, &shared_state.config);
    let option = shared_state.db.get(component.i).await.build(&shared_state.config.label_location, &shared_state.config);

    if let Some(bytes) = option {

        //let help = bytes.as_slice();

        let array = Bytes::from_owner(bytes);
        println!("returned some bytes ig");
        return array;
    } else {
        return Bytes::new();
    }
    //shared_state.db.get_first().await.build(&shared_state.config.label_location);

    
}