use std::sync::Arc;

use axum::{debug_handler, extract::State, response::{IntoResponse, Redirect}, Form, Json};
use crate::{db::{self, components::{Component, ComponentServices}}, server::server_state::ServerState};



pub struct PostComponent {
    component: Component,
    image: Option<Vec<u8>>,
    datasheet: Option<Vec<u8>>,

}


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<Component>,
) -> impl IntoResponse {

    // println!("IM HERE!");

    // println!("{}", c.fmt());

    //c.optimise_image();

    shared_state.db.add(c, &shared_state.config).await;

    Redirect::to("/").into_response()
}