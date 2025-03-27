use crate::{db::components::Components, Config};

pub mod login_api;
pub mod db_api;

use db_api::{get_all_component, get_first_component, post_component};

use axum::{
    extract::Query, http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{any_service, get, get_service, post}, Form, Json, Router
};

use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder};
use login_api::login::{Backend,User};
use login_api::handler;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

pub async fn start_server(config: Config, db: Components) {

    println!("{0}",config.password);


    

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(User {username: config.user, password: config.password});
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> = AuthManagerLayerBuilder::new(backend, session_layer).build();


    let shared_state = Arc::new(db);



    //#[cfg(debug_assertions)]
    //let app = get_router_debug();

    //#[cfg(not(debug_assertions))]
    //let app = get_router(auth_layer);


    let api_router = Router::new()
        .route("/", get(handler))
        .route("/post_component", post(post_component::post_component))
        .route("/get_first_component", get(get_first_component::get_component))
        .route("/get_all_component", get(get_all_component::get_component));

    let app = Router::new()
        
        
        

        .route("/", any_service(ServeDir::new("dist/src")))
        .route_layer(login_required!(Backend, login_url = "/login"))


        



        .nest("/api", api_router)
        .route_layer(login_required!(Backend, login_url = "/login"))
        


        .route("/login_api", post(handler::login))

        .nest_service("/login", ServeDir::new("dist/login"))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        
        .route("/logout", get(handler::logout))
        
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
        )
        
        .with_state(shared_state);


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