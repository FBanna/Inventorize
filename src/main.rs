
use std::{io::Error, sync::{Arc, atomic::AtomicBool}};

use crate::{config::config::Config, db::{components::Component, types::{service::ComponentTypeService, transport_type::TransportComponentType}}};
use db::{components::ComponentServices, db::DB};
use serde_json::json;
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


    let test_type = TransportComponentType{
        name: "resistor".to_owned(),
        inherits: 1,
        attributes: Some(json!({

            "attributes": [
                {
                    "name": "resistance",
                    "object_type": "integer",
                    "unit": "R"
                },
                {
                    "name": "package",
                    "object_type": "string",
                    "unit": ""
                }

            ]
            
        }))
    };


    let result = component_db.add_type(&test_type).await.unwrap();

    let test_component = Component { 
        id: None, 
        name: "Boring Old Resistor".to_owned(), 
        stock: 1000, 
        price: Some(14.0), 
        manufacturer: Some("lcsc".to_owned()), 
        label: Some("vial".to_owned()), 
        image: false, 
        datasheet: false, 
        //attribute_id: result.last_insert_rowid() as i32, 
        attributes: json!({

            "attributes": [
                {
                    "id": result.last_insert_rowid() as i32,
                    "values": {

                        "resistance": 60,
                        "package": "0402"

                    }
                },
            ]
            

        })
    };

    component_db.add(&test_component).await.unwrap();


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