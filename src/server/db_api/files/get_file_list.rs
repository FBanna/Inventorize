use std::sync::Arc;

use axum::{Json, extract::{Multipart, Path, State}, response::IntoResponse};
use uuid::Uuid;
use crate::{db::component::properties::{file::{file::ComponentFile, service::ComponentFileService}, files::{self, DownloadedFile}}, error::error::AppError, server::server_state::ServerState};

pub async fn get_file_list(

    State(shared_state): State<Arc<ServerState>>,
    Json(component_id): Json<Uuid>
) -> Result<Json<Vec<ComponentFile>>, AppError> {


    let files = shared_state.db.get_all_files(component_id).await?;


    
    Ok(Json(files))

}

