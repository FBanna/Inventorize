use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{db::origin::{origin::Origin, service::OriginServices}, error::error::AppError, server::server_state::ServerState};



pub async fn get_all_origins(

    State(shared_state): State<Arc<ServerState>>

) -> Result<Json<Vec<Origin>>, AppError> {

    let result = shared_state.db.get_all_origins().await?;

    Ok(Json(result))
}
