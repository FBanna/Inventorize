use crate::{Config, db::{component::component::Component, db::DB}, server::{db_api::{class::{get_all_classes::get_all_classes, post_class_instance_id_get_class::post_class_instance_id_get_class}, class_instance::{get_class_instance_descendants::post_id_get_class_instance_descendants, post_class_instance::post_class_instance}, component::{post_component::{self, post_component}, post_search_get_component_with_attributes::post_search_get_component_with_attributes, post_search_get_facets::post_search_get_facets}}, embedded_dir::embedded_dir, label_api::post_build_label}};

// mod login_api;
// pub mod login_api;
// pub mod db_api;
// pub mod server_state;


use axum::{
    Form, Json, Router, extract::{DefaultBodyLimit, Query}, http::{HeaderValue, Method, StatusCode, Uri, header::CONTENT_TYPE}, response::{Html, IntoResponse, Redirect}, routing::{any_service, get, get_service, post}
};

use axum_login::{login_required, predicate_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayer, AuthManagerLayerBuilder, AuthSession, AuthUser};
use rust_embed::Embed;
use super::login_api::login::{Backend,User};
use super::login_api::handler;
use super::server_state::ServerState;
use typst::foundations::ops::pos;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};


pub async fn start_server(config: Config, db: DB) -> tokio::task::JoinHandle<()> {


    // login stuff
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(User {username: config.user.to_owned(), password: config.password.to_owned()});
    let auth_layer: AuthManagerLayer<Backend, MemoryStore> = AuthManagerLayerBuilder::new(backend, session_layer).build();

    
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port)); // could add custom port


    let shared_state = Arc::new(
        ServerState{
            db,
            config
        }
    );

    //static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR");
    // static IMAGES: Dir = include_dir!("$CARGO_TARGET_DIR/dist/images");

    let prot_frontend: Router<Arc<ServerState>>;




    // RELEASE BUILD
    //#[cfg(not(debug_assertions))]
    {

        #[derive(Embed)]
        #[folder = "./target/dist/assets"]
        struct Assets;

        #[derive(Embed)]
        #[folder = "./target/dist/images"]
        struct Images;

        let LOGIN = include_str!("../../target/dist/login.html");
        // static ASSETS: Dir = include_dir!("../dist/assets");
        // static IMAGES: Dir = include_dir!("../dist/images");

        prot_frontend = Router::new()
            .route("/login", get(|| async {
                Html(LOGIN).into_response()
            }))
            .nest_service("/assets", embedded_dir::<Assets>());
            // .nest_service("/login", ServeDir::new("./dist/login.html"))
            // .nest_service("/assets", ServeDir::new("./dist/assets"))
            // .nest_service("/images", ServeDir::new("./dist/images"));

    }
    

    // DEBUG BUILD
    #[cfg(debug_assertions)]
    {
        prot_frontend = Router::new()
        .nest_service("/login", ServeDir::new("../dist/login.html"))
        .nest_service("/assets", ServeDir::new("../dist/assets"))
        //.nest_service("/test", Se)
        .nest_service("/images", ServeDir::new("../dist/images"));

    }

    

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
        .layer(DefaultBodyLimit::disable()) 
        
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
        //.route("/post_build_zip", post(post_build_label::post_build_label))


        // COMPONENT

        .route("/post_component", post(post_component))
        .route("/post_search_get_component_with_attributes", post(post_search_get_component_with_attributes))
        .route("/post_search_get_facets", post(post_search_get_facets))
        
        // CLASS INSTANCE
        .route("/post_class_instance", post(post_class_instance))
        
        .route("/post_id_get_class_instance_descendants", post(post_id_get_class_instance_descendants))


        // CLASS
        .route("/get_all_classes", get(get_all_classes))
        .route("/post_class_instance_id_get_class", post(post_class_instance_id_get_class));

        // .route("/post_update_component", post(post_update_component::post_update_component))
        // .route("/post_build_label", post(post_build_label::post_build_label))
        // .route("/post_search_component", post(post_search_get_component))
        // .route("/post_id_get_component", post(post_id_get_component::post_id_get_component))
        // .route("/post_id_remove_component", post(post_id_remove_component::post_id_remove_component))
        // .route("/post_id_remove_list_component", post(post_id_remove_list_component))
        // .route("/get_first_component", get(get_first_component::get_component))
        // .route("/get_all_component", get(get_all_component::get_component))
        // .route("/post_type_id_get_prompts", get(post_type_id_get_prompts::post_type_id_get_prompts));
    

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

 