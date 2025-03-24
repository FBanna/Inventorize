use crate::cli::config::Config;
use log::{debug};


mod server;
mod cli;
mod db;

#[tokio::main]
async fn main() {

    env_logger::init();

    let config: Config = cli::get_config().await;

    config.print();


    let component_db = db::components::Component::init(&config.db_location).await;

    
    Config::debug_print(&config);

    server::start_server(config).await;
}
