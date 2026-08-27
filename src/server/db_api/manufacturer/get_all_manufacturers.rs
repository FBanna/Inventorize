use std::{result, sync::Arc};

use axum::{extract::State, Json};
use crate::{db::{class::{class::Class, service::ClassServices, transport_class::TransportClass}, component::{component::Component, service::ComponentServices}, manufacturer::{manufacturer::Manufacturer, service::ManufacturerServices}}, error::error::AppError, server::server_state::ServerState};



pub async fn get_all_manufacturers(

    State(shared_state): State<Arc<ServerState>>

) -> Result<Json<Vec<Manufacturer>>, AppError> {

    let result = shared_state.db.get_all_manufacturer().await?;

    Ok(Json(result))
}