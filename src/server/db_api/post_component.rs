use std::sync::Arc;

use axum::{debug_handler, extract::State, http::{Response, StatusCode}, response::{IntoResponse, Redirect}, Form, Json};
use crate::{db::{self, components::{Component, ComponentServices}, transport::post_component::PostComponent}, server::server_state::ServerState};


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<PostComponent>,
) -> impl IntoResponse {

    // println!("IM HERE!");

    // println!("{}", c.fmt());

    //c.optimise_image();

    shared_state.db.add_with_files(c, &shared_state.config).await;

    //shared_state.db.add(c, &shared_state.config).await;

    StatusCode::OK.into_response()

    //Redirect::to("/").into_response()
}