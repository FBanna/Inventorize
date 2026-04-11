// use std::sync::Arc;

// use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use crate::{db::{components::ComponentServices, transport::post_component::PostComponent}, error::error::AppError, server::server_state::ServerState};


// pub async fn post_component(

//     State(shared_state): State<Arc<ServerState>>,
//     Json(c): Json<PostComponent>,
// ) -> Result<impl IntoResponse, AppError> {

//     // println!("IM HERE!");

//     // println!("{}", c.fmt());

//     //c.optimise_image();

//     shared_state.db.add_with_files(c, &shared_state.config).await?;

//     //shared_state.db.add(c, &shared_state.config).await;

//     Ok(StatusCode::OK.into_response())

//     //Redirect::to("/").into_response()
// }

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{db::{component::service::ComponentServices, types::{component_type_value::ComponentTypeValue, service::ComponentTypeService, transport_type::TransportComponentType}}, error::error::AppError, server::server_state::ServerState};


// pub struct TransportComponentTypeValue {
//     pub component_id: i32,
//     pub type_values: Vec<ComponentTypeValue>
//     pub type_id: i32,
//     pub attributes: serde_json::Value
// }

pub async fn post_component_type_values(
    State(shared_state): State<Arc<ServerState>>,
    Json(tc): Json<Vec<ComponentTypeValue>>
) -> Result<impl IntoResponse, AppError> {

    println!("Tying hard to add your type");

    let result = shared_state.db.add_component_type_values(tc).await;

    // TODO

    Ok(StatusCode::OK.into_response())



}