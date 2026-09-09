use std::path::Path;
use serde::Serialize;
use uuid::Uuid;

use crate::{config::config::Config, db::{component::transport_component::EmbeddedComponentOrigin, origin::origin::Origin}, error::{error::AppError::{self}, python::PythonErrors}, python::python_wrapper::{DataFromPython, run_python}};





#[derive(Serialize)]
pub struct ComponentFromPython {
    pub name: String,
    pub origin: EmbeddedComponentOrigin
    // image, datasheets, manufacturer, attributes
}




pub fn qr_python(qr: String, origin: Origin, config: &Config) -> Result<ComponentFromPython, AppError> {

    let result = run_python_get_component(origin.py_qr, origin.origin_id, config, qr)?;

    Ok(result)

}


fn run_python_get_component(option_path: Option<String>, origin_id: Uuid, config: &Config, data: String) -> Result<ComponentFromPython, AppError> {

    let path_str = option_path.ok_or(PythonErrors::NoFile)?;

    let path = Path::new(&path_str);

    let result: DataFromPython = run_python(path, config, data)?;




    let component = ComponentFromPython {
        name: result.name,
        origin: EmbeddedComponentOrigin {
            origin_id: origin_id,
            part_number: result.part_number,
            price: result.price
        }
    };

    Ok(component)


}