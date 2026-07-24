
use std::{collections::HashMap, io::Error, println, sync::{Arc, atomic::AtomicBool}};

use crate::{config::config::Config, db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{service::ClassInstanceServices, transport_class_instance::TransportClassInstance}, component::{component::Component, transport_component::{EmbeddedComponentClassAttributes, TransportComponent}}}};
use db::{component::service::ComponentServices, db::DB};
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


    let passive_class = TransportClass {
        name: "passives".to_owned(),
        fields: json!({
            "attributes": []
        })
    };

    let resistor_class = TransportClass {
        name: "resistor".to_owned(),
        fields: json!({
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
        }) 
    };

    let passive_class_id = component_db.add_transport_class(passive_class).await.unwrap();
    let resistor_class_id = component_db.add_transport_class(resistor_class).await.unwrap();

    let passive_class_instance = TransportClassInstance {
        class_id: passive_class_id,
        parent: None
    };


    let passive_class_instance_id = component_db.add_transport_class_instance(passive_class_instance).await.unwrap();

    let resistor_class_instance = TransportClassInstance {
        class_id: resistor_class_id,
        parent: Some(passive_class_instance_id)
    };

    let resistor_class_instance_id = component_db.add_transport_class_instance(resistor_class_instance).await.unwrap();

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




    // let t_id = component_db.add_type(&test_type).await.unwrap();

    // let test_component = Component { 
    //     id: 0, 
    //     name: "Boring Old Resistor".to_owned(), 
    //     stock: 1000, 
    //     price: Some(14.0), 
    //     manufacturer: Some("lcsc".to_owned()), 
    //     label: Some("vial".to_owned()), 
    //     image: false, 
    //     datasheet: false, 
    //     //attribute_id: result.last_insert_rowid() as i32, 
    //     // attributes: json!({

    //     //     "attributes": [
    //     //         {
    //     //             "id": result.last_insert_rowid() as i32,
    //     //             "values": {

    //     //                 "resistance": 60,
    //     //                 "package": "0402"

    //     //             }
    //     //         },
    //     //     ]
            

    //     // })
    // };

    // let c_id = component_db.add(&test_component).await.unwrap();

    // let test_component_type = ComponentTypeValue {
    //     component_id: c_id,
    //     type_id: t_id,
    //     attributes: json!({
    //         "resistor": 20,
    //         "package": "0402"
    //     })


    // };

    // let result2 = component_db.add_component_type_value(test_component_type).await.unwrap();




    let join_handle = server::entry::start_server(config, component_db).await; 


    signal::ctrl_c().await?;

    println!("Ctrl-C - EXITING");

    pool_clone.close().await;


    Ok(())

}