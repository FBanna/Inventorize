use std::{println, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{service::ClassInstanceServices, transport_class_instance::TransportClassInstance}}, error::error::AppError, server::server_state::ServerState};


pub async fn post_class_instance(
    State(shared_state): State<Arc<ServerState>>,
    Json(tc): Json<TransportClassInstance>
) -> Result<impl IntoResponse, AppError> {

    let result = shared_state.db.add_transport_class_instance(tc).await?;

    // TODO

    Ok(StatusCode::OK.into_response())



}