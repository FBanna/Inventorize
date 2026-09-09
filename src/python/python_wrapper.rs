use std::{ffi::CString, fs, path::{Path, PathBuf}};

use pyo3::{Py, PyAny, PyErr, PyResult, Python, exceptions::PyModuleNotFoundError, types::{PyAnyMethods, PyList, PyListMethods, PyModule}};

use crate::{config::config::Config, error::{error::AppError, python::PythonErrors}};


pub fn run_python(path: &Path, config: &Config, data: String) -> Result<(), AppError> {

    let path = PathBuf::from(config.python_location.clone()).join(path);


    let file_name = path.file_name()
        .ok_or(PythonErrors::PathError)?
        .to_owned()
        .into_string()
        .map_err(|_| PythonErrors::PathError)?;

    let file_name_c_string = CString::new(file_name.clone()).map_err(|_| PythonErrors::PathError)?;


    let py_app = CString::new(
        fs::read_to_string(&path).map_err(|_| PythonErrors::MissingFile(file_name.clone()))?
    ).map_err(|_| PythonErrors::PathError)?;

    println!("{:#?}", py_app);


    Python::initialize();


    let from_python = Python::attach(|py| -> PyResult<Py<PyAny>> {

        // let syspath = py
        //     .import("sys")?
        //     .getattr("path")?
        //     .cast_into::<PyList>()?;

        // syspath.insert(0, path)?;
        
        let app = PyModule::from_code(py, py_app.as_c_str(), file_name_c_string.as_c_str(), c"main")?.getattr("main")?;


        


        if app.is_callable() {

            let runner: Py<PyAny> = app.into();

            return runner.call0(py)
        } else {
            
            return Err(PyModuleNotFoundError::new_err("could not find function"))

        }

        
    })?;


    println!("py: {}", from_python);

    Ok(())

}

// pub fn t(path: &Path, config: &Config, variables: VariableSet) -> Result<HurlResult, AppError> {

//     let path = PathBuf::from(config.hurl_location.clone()).join(path);

//     let content = fs::read_to_string(&path)?;

//     let input = Input::from(path);

//     let runner_options = RunnerOptionsBuilder::new()
//         .build();

    
//     let logger_options = LoggerOptionsBuilder::new()
//         .verbosity(Some(Verbosity::Verbose))
//         .build();

//     let result = hurl::runner::run(
//         &content, 
//         Some(&input), 
//         &runner_options, 
//         &variables, 
//         &logger_options
//     ).map_err(|e| PythonErrors::Run(e))?;

//     if !result.success {

        

//         return Err(PythonErrors::Run("ERR runnig".to_owned()).into());

//     }

    
//     Ok(result)
// }