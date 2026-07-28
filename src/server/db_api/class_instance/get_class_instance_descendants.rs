use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{class_instance::ClassInstanceTree, service::ClassInstanceServices, transport_class_instance::TransportClassInstance}}, error::error::AppError, server::server_state::ServerState};

#[derive(Deserialize)]
pub struct ClassInstanceID{
    class_instance_id: Option<Uuid>
}

pub async fn post_id_get_class_instance_descendants(
    State(shared_state): State<Arc<ServerState>>,
    Json(id): Json<ClassInstanceID>
) -> Result<Json<Vec<ClassInstanceTree>>, AppError> {

    let result = shared_state.db.get_class_instance_descendants(id.class_instance_id).await?;

    Ok(Json(result))

}