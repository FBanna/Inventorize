use std::sync::Arc;

use axum::{extract::{Multipart, State}, response::IntoResponse};
use crate::{db::component::properties::files, error::error::AppError, server::server_state::ServerState};


pub async fn post_file(

    State(shared_state): State<Arc<ServerState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {

    files::add_file(multipart, &shared_state.config, &shared_state.db).await?;


    Ok(())

}

