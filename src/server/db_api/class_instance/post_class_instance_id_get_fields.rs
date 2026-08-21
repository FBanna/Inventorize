use std::{println, sync::Arc};

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{service::ClassInstanceServices, transport_class_instance::TransportClassInstance}}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ClassInstanceID{
    class_instance_id: Option<Uuid>
}

pub async fn post_class_instance_id_get_fields(
    State(shared_state): State<Arc<ServerState>>,
    Json(id): Json<ClassInstanceID>
) -> Result<Json<Value>, AppError> {

    let result = shared_state.db.build_facet_list_for_instance(id.class_instance_id).await?;

    // TODO

    Ok(Json(result))



}