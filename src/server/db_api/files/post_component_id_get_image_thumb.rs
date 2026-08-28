use std::{result, sync::Arc};

use axum::{Json, body::{Body, Bytes}, extract::State, http::Response, response::IntoResponse};
use serde::Deserialize;
use tower_http::body::Full;
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, properties::image::service::ComponentImageService, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    component_id: Uuid
}

pub async fn post_component_id_get_image_thumb(

    State(shared_state): State<Arc<ServerState>>,
    Json(id): Json<ComponentID>
) -> Result<impl IntoResponse, AppError> {

    let result = shared_state.db.get_thumb(id.component_id).await?;

    


    
    let builder = Response::builder()
        .header("Content-Type", "image/avif")
        .status(200)
        .body(Body::from(Bytes::from(result))).unwrap();


    Ok(builder)
}
