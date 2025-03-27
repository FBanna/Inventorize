use std::sync::Arc;

use axum::{extract::State, Json};
use crate::db::components::{Component, Components};


pub async fn get_component(
    State(shared_state): State<Arc<Components>>,
) -> Json<Vec<Component>>{

    let result = shared_state.get_all().await;

    Json(result)
}