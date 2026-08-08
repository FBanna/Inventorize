use std::{result, sync::Arc};

use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ClassInstanceID{
    class_instance_id: Uuid
}

pub async fn post_class_instance_id_get_class(

    State(shared_state): State<Arc<ServerState>>,
    Json(id): Json<ClassInstanceID>
) -> Result<Json<Class>, AppError> {

    let result = shared_state.db.get_class_from_class_instance(id.class_instance_id).await?;

    Ok(Json(result))
}
