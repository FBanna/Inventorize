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


    let component_db = db::components::Components::init(&config.db_location).await;

    let component = db::components::Component{
        //ID:5000,
        NAME:("Resistor".to_string()),
        SIZE:Some("0402".to_string()),
        INFO:Some("60 OHM".to_string()),
        STOCK:5000,
        ORIGIN:None, 
        URL: None
    };

    component_db.add(component).await;

    let component = db::components::Component{
        //ID:5000,
        NAME:("Resistor".to_string()),
        SIZE:Some("0603".to_string()),
        INFO:Some("60 OHM".to_string()),
        STOCK:10,
        ORIGIN:None, 
        URL: None
    };

    component_db.add(component).await;

    
    Config::debug_print(&config);

    server::start_server(config).await;
}
