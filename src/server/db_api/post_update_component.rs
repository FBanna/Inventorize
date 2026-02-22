use std::sync::Arc;

use axum::{extract::State, http::{StatusCode}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{db::{components::ComponentServices, transport::post_component::PostComponent}, error::error::AppError, server::server_state::ServerState};

#[derive(Serialize, Deserialize)]
pub struct PostUpdateComponent {
    pub id: i64,
    pub component: PostComponent,
}

pub async fn post_update_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<PostUpdateComponent>,
) -> Result<impl IntoResponse, AppError> {


    shared_state.db.update_with_files(c.id, c.component, &shared_state.config).await?;

    Ok(StatusCode::OK.into_response())

}