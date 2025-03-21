use crate::{Config};

pub mod login;
mod handler;

use axum::{
    extract::Query, http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{any_service, get, get_service, post}, Form, Json, Router
};

use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder};
use login::{Backend, User};
use std::net::SocketAddr;
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

pub async fn start_server(config: Config) {

    println!("{0}",config.password);


    

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(User {id: 0, username: config.user, password: config.password});
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> = AuthManagerLayerBuilder::new(backend, session_layer).build();



    //#[cfg(debug_assertions)]
    //let app = get_router_debug();

    //#[cfg(not(debug_assertions))]
    //let app = get_router(auth_layer);

    let app = Router::new()
        
        
        

        .route("/", any_service(ServeDir::new("dist/src")))

        //.fallback_service(ServeDir::new("dist/src"))

        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/api", get(handler))
        

        .nest_service("/login", ServeDir::new("dist/login"))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        //.route("/login", any_service(ServeDir::new("dist/login")))
        .route("/api/login", post(handler::login))
        .route("/logout", get(handler::logout))

        //.route("/test", get(|| async { "hi" }))
        //.fallback_service(ServeDir::new("dist"))

        
        
        

        
        //.route("/login", any_service(ServeDir::new("dist/login")))
        
        
        
        .layer(auth_layer)
        //
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

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}