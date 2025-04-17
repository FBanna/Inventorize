use crate::cli::config::Config;
use db::{components::ComponentServices, db::DB};
use label::label::Label;


mod server;
mod cli;
mod db;
mod label;

#[tokio::main]
async fn main() {

    let config: Config = cli::get_config().await;

    config.print();


    let mut component_db = DB::init(&config.db_location).await;

    let component = db::components::Component{
        //ID:5000,
        id: None,
        name:("Resistor".to_string()),
        size:Some("0402".to_string()),
        value:Some("60 OHM".to_string()),
        info:None,
        stock:5000,
        origin:None, 
        //url: None,
        label: Some("vial".to_string())
    };

    //component.build();

    //label::label::Label::new(component).build();

    component_db.add(component).await;

    let component = db::components::Component{
        //ID:5000,
        id: None,
        name:("Resistor".to_string()),
        size:Some("0603".to_string()),
        value:Some("180 OHM".to_string()),
        info:Some("Thick Film".to_string()),
        stock:10,
        origin:None, 
        //url: None,
        label: None
    };

    component_db.add(component).await;

    
    Config::debug_print(&config);

    server::start_server(config, component_db).await;
}
