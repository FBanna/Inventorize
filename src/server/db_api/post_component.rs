use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{db::{components::ComponentServices, transport::post_component::PostComponent}, error::error::AppError, server::server_state::ServerState};


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<PostComponent>,
) -> Result<impl IntoResponse, AppError> {

    // println!("IM HERE!");

    // println!("{}", c.fmt());

    //c.optimise_image();

    shared_state.db.add_with_files(c, &shared_state.config).await?;

    //shared_state.db.add(c, &shared_state.config).await;

    Ok(StatusCode::OK.into_response())

    //Redirect::to("/").into_response()
}