use std::{any::Any, collections::HashMap, iter::Map, path::Path, result};

use axum_login::tracing::field;
use serde::Serialize;
use serde_json::Value as Json;
use uuid::Uuid;

use crate::{config::config::Config, db::{component::transport_component::EmbeddedComponentOrigin, origin::origin::Origin}, error::{error::AppError::{self}, python::PythonErrors}, python::python_wrapper::run_python};



#[derive(Serialize)]
pub struct ComponentFromPython {
    pub name: String,
    pub origin: EmbeddedComponentOrigin
    // image, datasheets, manufacturer, attributes
}

pub fn qr_python(qr: String, origin: Origin, config: &Config) -> Result<ComponentFromPython, AppError> {




    // match on json
    //serde_json::from_str(s)

    println!("json: {}", qr);
    //let test: Json = serde_json::from_str::<Json>(&qr).unwrap();


    // if let Ok(json) = serde_json::from_str::<Json>(&qr) {
    //     println!("its json all right!");


    //     println!("extracted {:#?}", json);

    //     if json.is_object() {
            
    //         let json_object = json.as_object().unwrap();

    //         for (key, value) in json_object {
    //             variables.insert(key.to_owned(), Value::from_json(value));
    //         }
    //     } else {
    //         let hurl_json = Value::from_json(&json);

    //         variables.insert("json".to_owned(), hurl_json);
    //     }





    // } else if let Ok(corrected_json) = serde_json::from_str::<Json>(&correct_json(&qr)) {

    //     println!("its json all right!");


    //     println!("extracted {:#?}", corrected_json);

    //     if corrected_json.is_object() {
            
    //         let json_object = corrected_json.as_object().unwrap();

    //         for (key, value) in json_object {
    //             variables.insert(key.to_owned(), Value::from_json(value));
    //         }
    //     } else {
    //         let hurl_json = Value::from_json(&corrected_json);

    //         variables.insert("json".to_owned(), hurl_json);
    //     }

    // } else {

    //     variables.insert("data".to_owned(), Value::String(qr));

    // }

    let result = run_python_get_component(origin.py_qr, origin.origin_id, config, qr)?;

    

    Ok(result)



}


fn run_python_get_component(option_path: Option<String>, origin_id: Uuid, config: &Config, data: String) -> Result<ComponentFromPython, AppError> {

    let path_str = option_path.ok_or(PythonErrors::NoFile)?;

    let path = Path::new(&path_str);

    let result = run_python(path, config, data);

    // let captures: Vec<CaptureResult> = result.entries.into_iter().flat_map(|e| e.captures).collect();

    // let mut map: HashMap<String, Value> = HashMap::new();


    // for capture in captures {
        
    //         map.insert(capture.name, capture.value);

    // }



    // let name = get_field_from_map::<String>(&map, "name".to_string())?;

    // let part_number = get_option_field_from_map::<String>(&map, "pn".to_string())?;
    // let price = get_option_field_from_map::<i32>(&map, "price".to_string())?;

    let component = ComponentFromPython {
        name: "test".to_string(),
        origin: EmbeddedComponentOrigin {
            origin_id: origin_id,
            part_number: None,
            price: None
        }
    };

    Ok(component)


}





// fn correct_json(input: &String) -> String {

//     let mut out = input.replace("{", "{\"");
//     out = out.replace(":", "\":\"");
//     out = out.replace(",", "\",\"");
//     out = out.replace("}", "\"}");

//     return out;
// }








// fn get_option_field_from_map<T: FromValue>(map: &HashMap<String, Value>, field: String) -> Result<Option<T>, AppError>
// {

//     //let value = map.get(&field).ok_or(HurlErrors::NoField(field.clone()))?;

//     if let Some(value) = map.get(&field) {
//         return Ok(Some(T::from_value(value, field)?))
//     }

//     return Ok(None);

// }

// fn get_field_from_map<T: FromValue>(map: &HashMap<String, Value>, field: String) -> Result<T, AppError>
// {

//     let value = map.get(&field).ok_or(PythonErrors::NoField(field.clone()))?;

//     return Ok(T::from_value(value, field)?)

// }



// pub trait FromValue: Sized {
//     fn from_value(value: &Value, field: String) -> Result<Self, AppError>;
// }

// impl FromValue for String {
//     fn from_value(value: &Value, field: String) -> Result<Self, AppError> {
//         match value {
//             Value::String(v) => Ok(v.to_owned()),
//             _ => Err(PythonErrors::ImproperField(field).into()),
//         }
//     }
// }

// impl FromValue for i32 {
//     fn from_value(value: &Value, field: String) -> Result<Self, AppError> {
        
//         // match value {

//         //     Value::Number(v) => Ok(v.)
            
//         // }

//         return Ok(1 as i32);
//     }
// }