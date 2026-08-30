use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::origin::{service::OriginServices, transport_origin::TransportOrigin}, error::error::AppError, server::server_state::ServerState};



pub async fn post_origin(
    State(shared_state): State<Arc<ServerState>>,
    Json(to): Json<TransportOrigin>
) -> Result<impl IntoResponse, AppError> {

    let _result = shared_state.db.add_transport_origin(to).await?;

    Ok(StatusCode::OK.into_response())



}