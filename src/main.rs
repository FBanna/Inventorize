use crate::cli::config::Config;
use log::{debug};


mod server;
mod cli;

#[tokio::main]
async fn main() {

    env_logger::init();

    let config: Config = cli::get_config();

    
    Config::debug_print(&config);

    server::start_server(config).await;
}
