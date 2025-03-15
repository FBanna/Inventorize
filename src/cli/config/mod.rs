use std::{fs::{self, File}, io::Read, path::PathBuf};
use log::{info,debug,log_enabled,Level};
use serde::{Deserialize, Serialize};


pub const DEFAULT_CONFIG_FILE: &str = "./config.yaml";
const DEFAULT_PORT_NUMBER: u16 = 3030;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config{
    pub port: u16
}

impl Config{
    pub fn new() -> Config {
        Config{
            port: DEFAULT_PORT_NUMBER
        }
    }

    pub fn print(&self){

        let string = serde_yaml::to_string(self).expect("Could not serialize");
        println!("config is {string}");
    }

    pub fn debug_print(&self){

        let string = serde_yaml::to_string(self).expect("Could not serialize");
        info!("config is {string}");
    }


    pub fn write(&self){
        let string = serde_yaml::to_string(self).expect("Could not serialize");

        fs::write(DEFAULT_CONFIG_FILE, string).expect("Failed to write to file!");
    }
}

pub fn read_config(path: PathBuf) -> Config{
    let data = fs::read_to_string(path).expect("Unable to read File!");

    serde_yaml::from_str(&data).expect("Unable to Deserialize!")
}

