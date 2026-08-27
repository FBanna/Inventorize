use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::{class::{service::ClassServices, transport_class::TransportClass}, label::{service::LabelServices, transport_label::TransportLabel}}, error::error::AppError, server::server_state::ServerState};


pub async fn post_label(
    State(shared_state): State<Arc<ServerState>>,
    Json(tl): Json<TransportLabel>
) -> Result<impl IntoResponse, AppError> {

    let result = shared_state.db.add_transport_label(tl).await?;

    Ok(StatusCode::OK.into_response())



}