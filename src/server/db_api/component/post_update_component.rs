use std::sync::Arc;

use axum::{extract::State, http::{StatusCode}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{db::{component::{component::Component, service::ComponentServices}}, error::error::AppError, server::server_state::ServerState};

// #[derive(Serialize, Deserialize)]
// pub struct PostUpdateComponent {
//     pub id: i32,
//     pub component: TransportComponent,
// }

pub async fn post_update_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<Component>,
) -> Result<impl IntoResponse, AppError> {

    shared_state.db.update_component(c.component_id, &c).await?;

    Ok(StatusCode::OK.into_response())

}