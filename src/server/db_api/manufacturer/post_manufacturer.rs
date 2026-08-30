use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::manufacturer::{service::ManufacturerServices, transport::TransportManufacturer}, error::error::AppError, server::server_state::ServerState};


pub async fn post_manufacturer(
    State(shared_state): State<Arc<ServerState>>,
    Json(tm): Json<TransportManufacturer>
) -> Result<impl IntoResponse, AppError> {

    let _result = shared_state.db.add_transport_manufacturer(tm).await?;

    Ok(StatusCode::OK.into_response())

}

