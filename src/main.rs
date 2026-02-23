
use std::{io::Error, sync::{Arc, atomic::AtomicBool}};

use crate::config::config::Config;
use db::{components::ComponentServices, db::DB};
use tokio::{signal, sync::broadcast};
// use tokio::signal;



// use signal_hook::consts::signal::*;

mod server;
mod config;
mod db;
mod label;
mod error;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let config: Config = config::command::get_config().await;

    let component_db = DB::init(&config.db_location).await;

    let pool_clone = Arc::clone(&component_db.pool);
   

    server::entry::start_server(config, component_db).await;


    signal::ctrl_c().await?;

    println!("Ctrl-C - EXITING");

    pool_clone.close().await;

    Ok(())

}




    // let component = db::components::Component{
    //     //ID:5000,
    //     id: None,
    //     name:("Resistor".to_string()),
    //     size:Some("0402".to_string()),
    //     value:Some("60 OHM".to_string()),
    //     info:None,
    //     stock:5000,
    //     origin:None, 
    //     //url: None,
    //     label: Some("vial".to_string()),
    //     image: false,
    //     datasheet: false
    // };

    // println!("start");

    // for i in 1..100 {
    //     component_db.add(&component).await;
        
    // }

    // println!("stop");



    

    // //component.build();

    // //label::label::Label::new(component).build();

    // component_db.add(&component).await;

    // let component = db::components::Component{
    //     //ID:5000,
    //     id: None,
    //     name:("Resistor".to_string()),
    //     size:Some("0603".to_string()),
    //     value:Some("180 OHM".to_string()),
    //     info:Some("Thick Film".to_string()),
    //     stock:10,
    //     origin:None, 
    //     //url: None,
    //     label: None,
    //     image: false,
    //     datasheet: false
    // };

    // component_db.add(&component).await;