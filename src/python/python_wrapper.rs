use std::{ffi::CString, fs, path::{Path, PathBuf}};

use pyo3::{FromPyObject, PyResult, Python, exceptions::PyModuleNotFoundError, types::{PyAnyMethods, PyModule}};

use crate::{config::config::Config, error::{error::AppError, python::PythonErrors}};


#[derive(FromPyObject)]
pub struct DataFromPython {
    #[pyo3(item)]
    pub name: String,

    #[pyo3(item, default)]
    pub part_number: Option<String>,

    #[pyo3(item, default)]
    pub price: Option<i32>
}


pub fn run_python(path: &Path, config: &Config, data: String) -> Result<DataFromPython, AppError> {

    let path = PathBuf::from(config.python_location.clone()).join(path);


    let file_name = path.file_name()
        .ok_or(PythonErrors::PathError)?
        .to_owned()
        .into_string()
        .map_err(|_| PythonErrors::PathError)?;


    let py_app = CString::new(
        fs::read_to_string(&path).map_err(|_| PythonErrors::MissingFile(file_name.clone()))?
    ).map_err(|_| PythonErrors::PathError)?;

    


    Python::initialize();



    let from_python = Python::attach(|py| -> PyResult<DataFromPython> {

        
        let app = PyModule::from_code(py, py_app.as_c_str(), c"", c"")?.getattr("main")?;


        if app.is_callable() {

            let data = app.call1((data,))?;

            let formatted: DataFromPython = data.extract()?;

            return Ok(formatted)
        } else {
            
            return Err(PyModuleNotFoundError::new_err("could not find function"))

        }
        
    })?;


    

    Ok(from_python)

}
