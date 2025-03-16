use crate::{Config};

pub mod login;
mod handler;

use axum::{http::{header::CONTENT_TYPE, HeaderValue, Method}, middleware, response::Response, routing::{get, post}, Json, Router};
use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder};
use login::{Backend, User};
use std::net::SocketAddr;
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

pub async fn start_server(config: Config) {


    

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(User {id: 0, username: config.user, password: config.password});
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> = AuthManagerLayerBuilder::new(backend, session_layer).build();



    //#[cfg(debug_assertions)]
    //let app = get_router_debug();

    //#[cfg(not(debug_assertions))]
    //let app = get_router(auth_layer);

    let app =     Router::new()
        
        .nest_service("/", ServeDir::new("dist"))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(todo!()))
        .route("/login", get(todo!()))
        .layer(auth);
        // .layer(
        //     tower_http::cors::CorsLayer::new()
        //         .allow_origin("/".parse::<HeaderValue>().unwrap())
        //         .allow_headers([CONTENT_TYPE])
        //         .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        // );

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    println!("Server started, listening on {addr}");

    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}


fn get_router_debug() -> Router{
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


#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}