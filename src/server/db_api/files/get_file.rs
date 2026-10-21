use std::{result, sync::Arc};

use axum::{Json, body::{Body, Bytes}, extract::{Path, State}, http::{Response, StatusCode}, response::IntoResponse};
use serde::Deserialize;
use tower_http::{body::Full};
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, properties::{file::service::ComponentFileService, image::service::ComponentImageService}, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};





pub async fn get_file(
    
    State(shared_state): State<Arc<ServerState>>,
    Path(component_id): Path<Uuid>,

) -> Result<impl IntoResponse, AppError> {

    //let result = shared_state.db.get_full(component_id).await?;

    let result = shared_state.db.get_file(component_id).await?;

    let stream = result.fetch_file_stream(&shared_state.config).await?;

    let builder = Response::builder()
        .header("Content-Type", result.mime)
        .status(200)
        .body(Body::from_stream(stream)).unwrap();


    return Ok(builder)

    
    


    
}
