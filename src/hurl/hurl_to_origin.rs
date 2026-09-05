use std::{collections::HashMap, iter::Map, path::Path, result};

use axum_login::tracing::field;
use hurl::runner::{CaptureResult, Value, VariableSet};
use serde::Serialize;
use serde_json::Value as Json;
use uuid::Uuid;

use crate::{config::config::Config, db::{component::transport_component::EmbeddedComponentOrigin, origin::origin::Origin}, error::{error::AppError::{self, HurlError}, hurl::HurlErrors}, hurl::hurl_wrapper::run_hurl};



#[derive(Serialize)]
pub struct ComponentFromHurl {
    pub name: String,
    pub origin: EmbeddedComponentOrigin
    // image, datasheets, manufacturer, attributes
}

pub fn qr_hurl(qr: String, origin: Origin, config: &Config) -> Result<ComponentFromHurl, AppError> {



    let mut variables: VariableSet = VariableSet::new();

    // match on json

    if let Ok(json) = serde_json::to_value(&qr) {
        println!("its json all right!");


        let hurl_json = Value::from_json(&json);

        variables.insert("json".to_owned(), hurl_json);


    } else {

        variables.insert("data".to_owned(), Value::String(qr));

    }

    let result = run_hurl_get_component(origin.hurl_qr, origin.origin_id, config, variables)?;

    

    Ok(result)



}


fn run_hurl_get_component(option_path: Option<String>, origin_id: Uuid, config: &Config, variables: VariableSet) -> Result<ComponentFromHurl, AppError> {

    let path_str = option_path.ok_or(HurlErrors::NoFile)?;

    let path = Path::new(&path_str);

    let result = run_hurl(path, config, variables)?;

    let captures: Vec<CaptureResult> = result.entries.into_iter().flat_map(|e| e.captures).collect();

    let mut map: HashMap<String, Value> = HashMap::new();


    for capture in captures {
        
            map.insert(capture.name, capture.value);

    }



    let name = get_field_from_map::<String>(&map, "name".to_string())?;

    let part_number = get_option_field_from_map::<String>(&map, "pn".to_string())?;
    let price = get_option_field_from_map::<i32>(&map, "price".to_string())?;

    let component = ComponentFromHurl {
        name: name,
        origin: EmbeddedComponentOrigin {
            origin_id: origin_id,
            part_number: part_number,
            price: price
        }
    };

    Ok(component)


}



fn get_option_field_from_map<T: FromValue>(map: &HashMap<String, Value>, field: String) -> Result<Option<T>, AppError>
{

    //let value = map.get(&field).ok_or(HurlErrors::NoField(field.clone()))?;

    if let Some(value) = map.get(&field) {
        return Ok(Some(T::from_value(value, field)?))
    }

    return Ok(None);

}

fn get_field_from_map<T: FromValue>(map: &HashMap<String, Value>, field: String) -> Result<T, AppError>
{

    let value = map.get(&field).ok_or(HurlErrors::NoField(field.clone()))?;

    return Ok(T::from_value(value, field)?)

}



pub trait FromValue: Sized {
    fn from_value(value: &Value, field: String) -> Result<Self, AppError>;
}

impl FromValue for String {
    fn from_value(value: &Value, field: String) -> Result<Self, AppError> {
        match value {
            Value::String(v) => Ok(v.to_owned()),
            _ => Err(HurlErrors::ImproperField(field).into()),
        }
    }
}

impl FromValue for i32 {
    fn from_value(value: &Value, field: String) -> Result<Self, AppError> {
        
        // match value {

        //     Value::Number(v) => Ok(v.)
            
        // }

        return Ok(1 as i32);
    }
}