use crate::{db::{components::Component, db::DB}, Config};

pub mod login_api;
pub mod db_api;
pub mod server_state;

use db_api::{get_all_component, get_all_prompt, get_first_component, post_build_label, post_build_label_zip::post_build_label_zip, post_component, post_id_get_component, post_search_component::post_search_component};

use axum::{
    extract::{DefaultBodyLimit, Query}, http::{header::CONTENT_TYPE, HeaderValue, Method, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{any_service, get, get_service, post}, Form, Json, Router
};

use axum_login::{login_required, predicate_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder, AuthSession, AuthUser};
use login_api::login::{Backend,User};
use login_api::handler;
use server_state::ServerState;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

pub async fn start_server(config: Config, db: DB) {

    println!("{0}",config.password);


    

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

    



    //#[cfg(debug_assertions)]
    //let app = get_router_debug();

    //#[cfg(not(debug_assertions))]
    //let app = get_router(auth_layer);

    

    let app = Router::new()

        .merge(protected())

        .route("/login_api", post(handler::login))

        .nest_service("/login", ServeDir::new("../dist/login/index.html"))
        .nest_service("/assets", ServeDir::new("../dist/assets"))
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

    let server = axum::serve(listener, app).await.unwrap();

}


fn api() -> Router<Arc<ServerState>>{
    let api: Router<Arc<ServerState>> = Router::new()
        .route("/", get(handler))
        .route("/post_component", post(post_component::post_component))
        .route("/post_build", post(post_build_label::post_build_label))
        .route("/post_build_zip", post(post_build_label_zip))
        .route("/post_search_component", post(post_search_component))
        .route("/post_id_get_component", post(post_id_get_component::post_id_get_component))
        .route("/get_first_component", get(get_first_component::get_component))
        .route("/get_all_component", get(get_all_component::get_component))
        .route("/get_all_prompt", get(get_all_prompt::get_all_prompt));
    

    return api;
}

fn protected() -> Router<Arc<ServerState>>{
    let service = ServeFile::new("../dist/index.html");
    let protected = Router::new()
        .route_service("/", service.clone())
        .route_service("/addcomponent", service.clone())
        .route_service("/component/{id}", service)
        .nest("/api", api());
        //.fallback_service(ServeFile::new("../dist/index.html"));

    #[cfg(not(debug_assertions))]
    let protected = protected.route_layer(login_required!(Backend, login_url = "/login"));


    return protected;
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