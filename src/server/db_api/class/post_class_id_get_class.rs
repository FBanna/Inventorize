use std::{result, sync::Arc};

use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ClassID{
    class_id: Uuid
}

pub async fn post_id_get_class(

    State(shared_state): State<Arc<ServerState>>,
    Json(id): Json<ClassID>
) -> Result<Json<Class>, AppError> {

    let result = shared_state.db.get_class(id.class_id).await?;

    Ok(Json(result))
}
