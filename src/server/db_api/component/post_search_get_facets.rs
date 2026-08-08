use std::{sync::Arc};

use axum::{extract::State, Json};
use serde_json::Value;
use crate::{db::component_class::{component_class::ComponentSearch, service::ComponentClassServices}, error::error::AppError, server::server_state::ServerState};




pub async fn post_search_get_facets(

    State(shared_state): State<Arc<ServerState>>,
    Json(search): Json<ComponentSearch>
) -> Result<Json<Option<Value>>, AppError> {

    let result = shared_state.db.get_facets_from_search_on_component_class(search).await?;



    Ok(Json(result.jsonb_agg))

}