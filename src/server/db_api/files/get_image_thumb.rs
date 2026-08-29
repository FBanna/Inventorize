use std::{result, sync::Arc};

use axum::{Json, body::{Body, Bytes}, extract::{Path, State}, http::{Response, StatusCode}, response::IntoResponse};
use serde::Deserialize;
use tower_http::{body::Full};
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, properties::image::service::ComponentImageService, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};



pub async fn get_image_thumb(
    
    State(shared_state): State<Arc<ServerState>>,
    Path(component_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let result = shared_state.db.get_thumb(component_id).await?;


    if let Some(image) = result {


        let builder = Response::builder()
            .header("Content-Type", "image/avif")
            .status(200)
            .body(Body::from(Bytes::from(image))).unwrap();


        return Ok(builder)


    }


    return  Ok(StatusCode::NO_CONTENT.into_response());
    


    
    


    
}
