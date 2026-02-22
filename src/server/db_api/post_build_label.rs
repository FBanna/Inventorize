use std::sync::Arc;

use axum::{body::Bytes, extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{db::components::{Component, ComponentServices}, error::error::AppError, label::label::Label, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct BuildLabelList{
    list: Vec<i32>
}

pub async fn post_build_label(

    State(shared_state): State<Arc<ServerState>>,
    Json(components): Json<BuildLabelList>
    
) -> Result<impl IntoResponse, AppError> {

    println!("1: starting");

    let list = shared_state.db.get_from_list(components.list).await?;

    println!("2");

    let result = Component::build_pdf(list, &shared_state.config)?;

    let array = Bytes::from_owner(result);


    return Ok(array.into_response())


}