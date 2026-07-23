use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::{db::component::{service::ComponentServices, transport_component::TransportComponent}, error::error::AppError, server::server_state::ServerState};


pub async fn post_component(

    State(shared_state): State<Arc<ServerState>>,
    Json(c): Json<TransportComponent>,
) -> Result<impl IntoResponse, AppError> {

    let id = shared_state.db.add_transport_component(&c).await?; // must fix this method, only adds it to component table!

    // old functions that should not be here!
    //let attributes = c.create_component_type_values(id);

    //shared_state.db.add_component_type_values(attributes).await?;


    Ok(StatusCode::OK.into_response())


}