use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{db::origin::{service::OriginServices, transport_origin::TransportOrigin}, error::error::AppError, hurl::hurl_to_origin::{ComponentFromHurl, qr_hurl}, server::server_state::ServerState};

#[derive(Deserialize)]
pub struct QRHurlToOrigin {
    qr: String,
    origin_id: Uuid
}


pub async fn post_qr_hurl_to_origin(
    State(shared_state): State<Arc<ServerState>>,
    Json(qr): Json<QRHurlToOrigin>
) -> Result<Json<ComponentFromHurl>, AppError> {


    let origin = shared_state.db.get_origin(qr.origin_id).await?;

    let result = qr_hurl(qr.qr, origin, &shared_state.config)?;

    


    Ok(Json(result))



}