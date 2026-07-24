use std::sync::Arc;

use axum::{extract::{Multipart, State}, response::IntoResponse};
use crate::{db::component::properties::{file::{file::ComponentFile, service::ComponentFileService}, files::{self, DownloadedFile}}, error::error::AppError, server::{db_api::files::util::{self}, server_state::ServerState}};

pub async fn post_file(

    State(shared_state): State<Arc<ServerState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {

    let file: DownloadedFile = util::get_file(multipart, &shared_state.config).await?;

    let c_file: ComponentFile = ComponentFile::new(file, &shared_state.config)?;

    shared_state.db.add_file(c_file).await?;

    
    Ok(())

}

