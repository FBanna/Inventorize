use std::{ffi::CString, fs, path::{Path, PathBuf}};

use pyo3::{Py, PyAny, PyResult, Python, types::{PyAnyMethods, PyList, PyListMethods, PyModule}};

use crate::{config::config::Config, error::{error::AppError, python::PythonErrors}};


pub fn run_python(path: &Path, config: &Config, data: String) -> PyResult<()> {

    let path = PathBuf::from(config.python_location.clone()).join(path);

    let py_app = CString::new(fs::read_to_string(&path)?)?;

    Python::initialize();


    let from_python = Python::attach(|py| -> PyResult<Py<PyAny>> {

        // let syspath = py
        //     .import("sys")?
        //     .getattr("path")?
        //     .cast_into::<PyList>()?;

        // syspath.insert(0, path)?;
        
        let app: Py<PyAny> = PyModule::from_code(py, py_app.as_c_str(), c"", c"")?
            .getattr("main")?
            .into();

        app.call0(py)
    });


    println!("py: {}", from_python?);

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