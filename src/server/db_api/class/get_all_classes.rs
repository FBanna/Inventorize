use std::{result, sync::Arc};

use axum::{extract::State, Json};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};



pub async fn get_all_classes(

    State(shared_state): State<Arc<ServerState>>

) -> Result<Json<Vec<Class>>, AppError> {

    let result = shared_state.db.get_all_classes().await?;

    Ok(Json(result))
}