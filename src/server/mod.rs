use crate::{Config};

pub mod login;
mod handler;

use axum::{
    extract::Query, http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, get_service, post}, Form, Json, Router
};

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

    let app = Router::new()
        
        
        //.route_service("/prot",ServeFile::new("dist/index.html"))
        //
        .route("/api", get(handler))
        
        .route("/",get(page))

        // .route_layer(login_required!(Backend, login_url = "/login"))
        
        // .nest_service("/login", ServeDir::new("dist/login"))
        
        // .route("/api/login", post(handler::login))
        
        // .layer(auth_layer)
        .layer(
            //tower_http::cors::CorsLayer::permissive()
            tower_http::cors::CorsLayer::new()
                .allow_origin(
                    [
                        "/".parse::<HeaderValue>().unwrap(), 
                        #[cfg(debug_assertions)]
                        "http://localhost:5173".parse::<HeaderValue>().unwrap()
                    ]
                )
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        );


    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));


    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], config.port)))
    .await
    .unwrap();
 
    println!("Server started, listening on {addr}");

    let server = axum::serve(listener, app).await.unwrap();
}



async fn page() -> Html<&'static str>{
    let html_content = include_str!("../../target/release/dist/index.html");
    Html(html_content)
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