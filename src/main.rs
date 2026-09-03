
use std::{collections::HashMap, io::Error, path::Path, println, sync::{Arc, atomic::AtomicBool}};

use crate::{config::config::Config, db::{class::{service::ClassServices, transport_class::TransportClass}, class_instance::{service::ClassInstanceServices, transport_class_instance::TransportClassInstance}, component::{component::Component, properties::origin::component_origin::ComponentOrigin, transport_component::{EmbeddedComponentClassAttributes, EmbeddedComponentOrigin, TransportComponent}}, component_class::{component_class::{FacetSearch, PagedComponentSearch, TablePageQuery, UnitComponentClassSearch}, service::ComponentClassServices}, label::{service::LabelServices, transport_label::TransportLabel}, manufacturer::{self, service::ManufacturerServices, transport::TransportManufacturer}, origin::{service::OriginServices, transport_origin::TransportOrigin}}, hurl::hurl_wrapper::run_hurl};
use db::{component::service::ComponentServices, db::DB};
use serde_json::json;
use tokio::{signal, sync::broadcast};
use serde_json::Value as Json;


mod server;
mod config;
mod db;
mod label;
mod hurl;
mod error;

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

    // Label

    let label = TransportLabel {
        name: "new".to_owned(),
        path: "test.typ".to_owned()
    };

    let label_id = component_db.add_transport_label(label).await.unwrap();

    // Manufacturer

    let manufacturer = TransportManufacturer {
        name: "ST".to_owned(),
        url: Some("stm.com".to_owned())
    };

    let man_id = component_db.add_transport_manufacturer(manufacturer).await.unwrap();

    // Origin

    let origin_t = TransportOrigin {
        name: "LCSC".to_owned(),
        url: Some("lcsc.com".to_owned()),
        hurl_get: None,
        hurl_price: None
    };

    let origin_id = component_db.add_transport_origin(origin_t).await.unwrap();


    // components
    let component1 = TransportComponent {
        class_instance_id: passive_class_instance_id,
        name: "test".to_owned(),
        stock: 5,
        manufacturer_id: None,
        label_id: None,
        attributes: HashMap::from([
            (passive_class_instance_id, json!({}))
        ]),
        origins: Vec::from(
            [EmbeddedComponentOrigin {
                origin_id,
                part_number: Some("xc3tr".to_owned()),
                price: Some(0.344 as i32)
            }]
        )
    };

    let component2 = TransportComponent {
        class_instance_id: resistor_class_instance_id,
        name: "some resistor".to_owned(),
        stock: 1000,
        manufacturer_id: Some(man_id),
        label_id: Some(label_id),
        attributes: HashMap::from([
            (
                resistor_class_instance_id, 
                json!({
                    "resistance": 60,
                    "package": "0402"
                })
            ),
            (
                passive_class_instance_id,
                json!({})
            )
        ]),
        origins: Vec::new()
    };


    for i in 0..1000 {
        println!("{}", i);
        let result = component_db.add_transport_component(&component1).await.unwrap();
        let result = component_db.add_transport_component(&component2).await.unwrap();
    }

    let result = component_db.add_transport_component(&component1).await.unwrap();
    let result = component_db.add_transport_component(&component2).await.unwrap();


    // search

    let search_unit = UnitComponentClassSearch {
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
        // facets: Vec::from([
        //     SearchFacet {
        //         key: "resistance".to_owned(),
        //         values: Vec::from([json!(60)])
        //     },
        //     SearchFacet {
        //         key: "package".to_owned(),
        //         values: Vec::from([json!("0402")])
        //     }
        // ])
    };

    let page_state = TablePageQuery {
        page_pos: 0,
        page_size: 50
    };

    let search = PagedComponentSearch {
            root: Some(resistor_class_instance_id),
            units: Vec::from([search_unit]),
            state: page_state
        };



    let search_result = component_db.search_components_with_attributes_on_component_class(

        search.clone()
        
    ).await;

    // if let Err(e) = search_result {
    //     println!("BIG ERROR FROM SEARCH {:#}", e);
    // } else {
    //     println!("{:#?}", search_result.unwrap());
    // }

    let facets = component_db.get_facets_from_search_on_component_class(

        FacetSearch {
            root: search.root,
            units: search.units
        }

    ).await;



    let result = run_hurl(Path::new("test.hurl"), &config).unwrap();

    // if let Err(e) = facets {
    //     println!("BIG ERROR {:#}", e);
    // } else {
    //     println!("{:#?}", facets.unwrap());
    // }


    





    let join_handle = server::entry::start_server(config, component_db).await; 


    signal::ctrl_c().await?;

    println!("Ctrl-C - EXITING");

    pool_clone.close().await;


    Ok(())

}