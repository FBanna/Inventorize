use crate::{Config};

use axum::{http::{header::CONTENT_TYPE, HeaderValue, Method}, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

pub async fn start_server(config: Config) {

    let app = get_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    println!("Server started, listening on {addr}");

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[cfg(debug_assertions)]
fn get_router() -> Router{
    Router::new()
        .route("/", get(handler))
        .layer(
            tower_http::cors::CorsLayer::permissive()
            // tower_http::cors::CorsLayer::new()
            //     .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            //     .allow_headers([CONTENT_TYPE])
            //     .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
}

#[cfg(not(debug_assertions))]
fn get_router() -> Router{
    
    Router::new()
        .route("/api", get(handler))
        .nest_service("/", ServeDir::new("dist"))//.not_found_service(ServeFile::new("dist/index.html")))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("/".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
}



#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}