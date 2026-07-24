use std::sync::Arc;

use axum::{extract::{Multipart, State}, response::IntoResponse};
use crate::{db::component::properties::{file::{file::ComponentFile, service::ComponentFileService}, files::{self, DownloadedFile}, image::{image::ComponentImage, service::ComponentImageService}}, error::error::AppError, server::{db_api::files::util::{self}, server_state::ServerState}};

pub async fn post_img(

    State(shared_state): State<Arc<ServerState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {

    let file: DownloadedFile = util::get_file(multipart, &shared_state.config).await?;

    let c_img: ComponentImage = ComponentImage::new(file)?;

    shared_state.db.add_img(c_img).await?;


    Ok(())

}

