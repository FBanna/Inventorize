use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::class::{service::ClassServices, transport_class::TransportClass}, error::error::AppError, server::server_state::ServerState};


pub async fn post_class(
    State(shared_state): State<Arc<ServerState>>,
    Json(tc): Json<TransportClass>
) -> Result<impl IntoResponse, AppError> {

    println!("Tying hard to add your class");

    let result = shared_state.db.add_transport_class(tc).await?;

    //let result = shared_state.db.add_type(&tc).await;

    // TODO

    Ok(StatusCode::OK.into_response())



}