use std::sync::Arc;

use axum::{extract::State, Json};
use crate::db::components::{Component, Components};


pub async fn get_component(
    State(shared_state): State<Arc<Components>>,
) -> Json<Component>{

    let result = shared_state.get().await;

    Json(result)
}