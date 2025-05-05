use std::sync::Arc;

use axum::{debug_handler, extract::State, http::{Response, StatusCode}, response::{IntoResponse, Redirect}, Form, Json};
use serde::{Deserialize, Serialize};
use crate::{db::{self, components::{Component, ComponentServices}, transport::post_component::PostComponent}, server::server_state::ServerState};

#[derive(Serialize, Deserialize)]
pub struct PostUpdateComponent {
    pub id: i64,
    pub component: PostComponent,
}

pub async fn post_update_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<PostUpdateComponent>,
) -> impl IntoResponse {

    // println!("IM HERE!");

    // println!("{}", c.fmt());

    //c.optimise_image();

    println!("trying to update!");

    shared_state.db.update_with_files(c.id, c.component, &shared_state.config).await;

    //shared_state.db.add(c, &shared_state.config).await;

    StatusCode::OK.into_response()

    //Redirect::to("/").into_response()
}