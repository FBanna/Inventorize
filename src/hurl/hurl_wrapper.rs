use std::{fs, path::{Path, PathBuf}};

use hurl::{runner::{RunnerOptionsBuilder, VariableSet}, util::logger::{ErrorFormat, LoggerOptionsBuilder, Verbosity}};
use hurl_core::input::Input;

use crate::{config::config::Config, error::error::AppError};




pub fn run_hurl(path: &Path, config: &Config) -> Result<(), AppError> {

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
    );

    Ok(())
}