use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Redirect}, Form, debug_handler};
use crate::{db::{self, components::{Component, ComponentServices}}, server::server_state::ServerState};


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Form(c): Form<Component>,
) -> impl IntoResponse {

    // println!("IM HERE!");

    // println!("{}", c.fmt());

    shared_state.db.add(c).await;

    Redirect::to("/").into_response()
}