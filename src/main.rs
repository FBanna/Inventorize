
use std::{collections::HashMap, io::Error, println, sync::{Arc, atomic::AtomicBool}};

use crate::{config::config::Config, db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{service::ClassInstanceServices, transport_class_instance::TransportClassInstance}, component::{component::Component, transport_component::{EmbeddedComponentClassAttributes, TransportComponent}}, component_class::{component_class::{ComponentSearch, UnitComponentClassSearch}, service::ComponentClassServices}}};
use db::{component::service::ComponentServices, db::DB};
use serde_json::json;
use tokio::{signal, sync::broadcast};
use serde_json::Value as Json;

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




    // MOVE TO UNIT TEST!



    // Define classes
    let passive_class = TransportClass {
        name: "passives".to_owned(),
        fields: json!([])
    };

    let resistor_class = TransportClass {
        name: "resistor".to_owned(),
        fields: json!(
            [
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
        ) 
    };

    // add classes
    let passive_class_id = component_db.add_transport_class(passive_class).await.unwrap();
    let resistor_class_id = component_db.add_transport_class(resistor_class).await.unwrap();

    // define class instance
    let passive_class_instance = TransportClassInstance {
        class_id: passive_class_id,
        parent: None
    };

    // add
    let passive_class_instance_id = component_db.add_transport_class_instance(passive_class_instance).await.unwrap();

    // define class instance
    let resistor_class_instance = TransportClassInstance {
        class_id: resistor_class_id,
        parent: Some(passive_class_instance_id)
    };

    // add
    let resistor_class_instance_id = component_db.add_transport_class_instance(resistor_class_instance).await.unwrap();


    // components
    let component1 = TransportComponent {
        class_instance_id: passive_class_instance_id,
        name: "test".to_owned(),
        stock: 5,
        manufacturer: None,
        label: Some("vial".to_owned()),
        attributes: HashMap::from([
            (passive_class_id, json!({}))
        ]),
        origins: Vec::new()
    };

    let component2 = TransportComponent {
        class_instance_id: resistor_class_instance_id,
        name: "some resistor".to_owned(),
        stock: 1000,
        manufacturer: None,
        label: None,
        attributes: HashMap::from([
            (
                resistor_class_id, 
                json!({
                    "resistance": 60,
                    "package": "0402"
                })
            ),
            (
                passive_class_id,
                json!({})
            )
        ]),
        origins: Vec::new()
    };

    let result = component_db.add_transport_component(&component1).await.unwrap();
    let result = component_db.add_transport_component(&component2).await.unwrap();


    // search

    let search = UnitComponentClassSearch {
        class_instance_id: resistor_class_instance_id,
        facets: HashMap::from([
            (
                "resistance".to_owned(),
                Vec::from([json!(60)])
            ),
            (
                "package".to_owned(),
                Vec::from([json!("0402")])
            )
        ])
    };

    let search_result = component_db.search_components_with_attributes_on_component_class(

        ComponentSearch {
            root: Some(resistor_class_instance_id),
            units: Vec::from([search])
        }
        
    ).await;

    if let Err(e) = search_result {
        println!("BIG ERROR {:#}", e);
    } else {
        println!("{:#?}", search_result.unwrap());
    }


    





    let join_handle = server::entry::start_server(config, component_db).await; 


    signal::ctrl_c().await?;

    println!("Ctrl-C - EXITING");

    pool_clone.close().await;


    Ok(())

}