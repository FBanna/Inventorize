use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::origin::service::OriginServices, error::error::AppError, python::python_to_origin::{ComponentFromPython, qr_python}, server::server_state::ServerState};


#[derive(Deserialize)]
pub struct QRHurlToOrigin {
    qr: String,
    origin_id: Uuid
}


pub async fn post_qr_python_to_origin(
    State(shared_state): State<Arc<ServerState>>,
    Json(qr): Json<QRHurlToOrigin>
) -> Result<Json<ComponentFromPython>, AppError> {


    let origin = shared_state.db.get_origin(qr.origin_id).await?;

    let result = qr_python(qr.qr, origin, &shared_state.config)?;


    Ok(Json(result))



}