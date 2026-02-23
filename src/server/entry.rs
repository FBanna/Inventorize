use crate::{db::{components::Component, db::DB}, Config};

// mod login_api;
// pub mod login_api;
// pub mod db_api;
// pub mod server_state;

use super::db_api::{get_all_component, get_all_prompt, get_first_component, post_build_label, post_component, post_id_get_component, post_id_remove_component, post_id_remove_list_component::post_id_remove_list_component, post_search_get_component::post_search_get_component, post_update_component};

use axum::{
    extract::{DefaultBodyLimit, Query}, http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{any_service, get, get_service, post}, Form, Json, Router
};

use axum_login::{login_required, predicate_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder, AuthSession, AuthUser};
use super::login_api::login::{Backend,User};
use super::login_api::handler;
use super::server_state::ServerState;
use typst::foundations::ops::pos;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};


pub async fn start_server(config: Config, db: DB) -> tokio::task::JoinHandle<()> {

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(User {username: config.user.to_owned(), password: config.password.to_owned()});
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> = AuthManagerLayerBuilder::new(backend, session_layer).build();

    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));


    let shared_state = Arc::new(
        ServerState{
            db,
            config
        }
    );

    



    // RELEASE BUILD
    #[cfg(not(debug_assertions))]
    let prot_frontend: Router<Arc<ServerState>> = Router::new()
        .nest_service("/login", ServeDir::new("./dist/login/index.html"))
        .nest_service("/assets", ServeDir::new("./dist/assets"));

    // DEBUG BUILD
    #[cfg(debug_assertions)]
    let prot_frontend: Router<Arc<ServerState>> = Router::new()
        .nest_service("/login", ServeDir::new("../dist/login/index.html"))
        .nest_service("/assets", ServeDir::new("../dist/assets"));


    

    let app = Router::new()

        .merge(protected())

        .route("/login_api", post(handler::login))

        .merge(prot_frontend)
        .nest_service("/data", ServeDir::new("./data"))

        .route("/logout", get(handler::logout))
        
        .layer(auth_layer)
        .layer(
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
        .layer(DefaultBodyLimit::disable()) // ALLOW LARGE FILE UPLOADS, NEED TO DO CHUNKED UPLOADS
        
        .with_state(shared_state);


    


    let listener = tokio::net::TcpListener::bind(addr)
    .await
    .unwrap();
 
    println!("Server started, listening on {addr}");

    let thread: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        let server = axum::serve(listener, app).await.unwrap(); //.with_graceful_shutdown(signal)

    });
    

    thread
    

}


fn api() -> Router<Arc<ServerState>>{
    let api: Router<Arc<ServerState>> = Router::new()
        //.route("/", get(handler))
        .route("/post_component", post(post_component::post_component))
        .route("/post_update_component", post(post_update_component::post_update_component))
        .route("/post_build_label", post(post_build_label::post_build_label))
        //.route("/post_build_zip", post(post_build_label::post_build_label))
        .route("/post_search_component", post(post_search_get_component))
        .route("/post_id_get_component", post(post_id_get_component::post_id_get_component))
        .route("/post_id_remove_component", post(post_id_remove_component::post_id_remove_component))
        .route("/post_id_remove_list_component", post(post_id_remove_list_component))
        .route("/get_first_component", get(get_first_component::get_component))
        .route("/get_all_component", get(get_all_component::get_component))
        .route("/get_all_prompt", get(get_all_prompt::get_all_prompt));
    

    return api;
}

fn protected() -> Router<Arc<ServerState>>{

    #[cfg(not(debug_assertions))]
    let service = ServeFile::new("./dist/index.html");
    #[cfg(debug_assertions)]
    let service = ServeFile::new("../dist/index.html");


    let protected = Router::new()
        .route_service("/", service.clone())
        .route_service("/addcomponent", service.clone())
        .route_service("/component/{id}", service.clone())
        .route_service("/component/{id}/update", service)
        .nest("/api", api());
    
    #[cfg(not(debug_assertions))]
    let protected = protected.route_layer(login_required!(Backend, login_url = "/login"));


    return protected;
}

// #[derive(serde::Serialize)]
// struct Message {
//     message: String,
// }

// async fn handler() -> Json<Message> {
//     Json(Message {
//         message: String::from("Hello, World!"),
//     })
// }