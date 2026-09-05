use std::{fs, path::{Path, PathBuf}};

use hurl::{runner::{HurlResult, RunnerOptionsBuilder, VariableSet}, util::logger::{ErrorFormat, LoggerOptionsBuilder, Verbosity}};
use hurl_core::input::Input;

use crate::{config::config::Config, error::{error::AppError, hurl::HurlErrors}};




pub fn run_hurl(path: &Path, config: &Config, variables: VariableSet) -> Result<HurlResult, AppError> {

    let path = PathBuf::from(config.hurl_location.clone()).join(path);

    let content = fs::read_to_string(&path)?;

    let input = Input::from(path);

    let runner_options = RunnerOptionsBuilder::new()
        .build();

    let variables = VariableSet::new();
    
    let logger_options = LoggerOptionsBuilder::new()
        .verbosity(Some(Verbosity::Verbose))
        .build();

    let result = hurl::runner::run(
        &content, 
        Some(&input), 
        &runner_options, 
        &variables, 
        &logger_options
    ).map_err(|e| HurlErrors::Run(e))?;

    
    Ok(result)
}