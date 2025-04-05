use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Redirect}, Form};
use crate::{db::{self, components::{Component, Components}}, server::server_state::ServerState};


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Form(c): Form<Component>,
) -> impl IntoResponse {

    println!("IM HERE!");

    println!("{}", c.fmt());

    shared_state.db.add(c).await;

    Redirect::to("/").into_response()
}