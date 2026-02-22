use std::sync::Arc;

use axum::{extract::State, Json};
use serde::Deserialize;
use crate::{db::components::{Component, ComponentServices}, error::error::AppError, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct ComponentID{
    i: i32
}

pub async fn post_id_get_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(component): Json<ComponentID>
) -> Result<Json<Component>, AppError> {


    
    //shared_state.db.get_first().await.build(&shared_state.config.label_location);

    let result = shared_state.db.get(component.i).await?;

    Ok(Json(result))
}