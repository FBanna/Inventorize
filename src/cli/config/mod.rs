use std::{fs::{self, File}, io::Read, path::PathBuf};
// use log::{info,debug,log_enabled,Level};
use serde::{Deserialize, Serialize};


pub const DEFAULT_CONFIG_FILE: &str = "./config.yaml";

const DEFAULT_DB_LOCATION: &str = "sqlite://inventorize.db";
const DEFAULT_PORT_NUMBER: u16 = 3030;
const DEFAULT_USER: &str = "user";
const DEFAULT_PASSWORD: &str = "password";
const DEFAULT_LABEL_LOCATION: &str = "labels";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config{
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_location: String,
    pub label_location: String
}

impl Config{
    pub fn new() -> Config {
        Config{
            port:           DEFAULT_PORT_NUMBER,
            user:           DEFAULT_USER.to_string(),
            password:       DEFAULT_PASSWORD.to_string(),
            db_location:    DEFAULT_DB_LOCATION.to_string(),
            label_location: DEFAULT_LABEL_LOCATION.to_string()
        }
    }

    pub fn print(&self){

        let string = serde_yaml::to_string(self).expect("Could not serialize");
        println!("config is {string}");
    }

    pub fn debug_print(&self){

        let string = serde_yaml::to_string(self).expect("Could not serialize");
        println!("config is {string}");
        // info!("config is {string}");
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

