use std::sync::Arc;

use axum::{Json, extract::{Multipart, Path, State}, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;
use crate::{db::component::properties::{file::{file::ComponentFile, service::ComponentFileService}, files::{self, DownloadedFile}}, error::error::AppError, server::server_state::ServerState};



#[derive(Deserialize)]
pub struct ComponentID{
    component_id: Uuid
}


pub async fn get_file_list(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<ComponentID>
) -> Result<Json<Vec<ComponentFile>>, AppError> {


    let files = shared_state.db.get_component_files(c.component_id).await?;


    
    Ok(Json(files))

}

