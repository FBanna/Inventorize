pub mod config;
use crate::db::db::DB;

use std::{fs, path::{Path, PathBuf}, process::exit};
use clap::{arg, command, value_parser, Command};
use config::{Config, read_config, DEFAULT_CONFIG_FILE};
use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};








pub async fn get_config() -> Config{
    let mut config: Config = Config::new();

    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "sets config file location"
            ).required(false).value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(
                -p --port <PORT> "sets port number"
            ).required(false).value_parser(value_parser!(i32))
        )
        .arg(
            arg!(
                -a --address <ADDRESS> "sets host address"
            ).required(false).value_parser(value_parser!(String))
        )
        .arg(
            arg!(
                -d --db <DB> "sets db location"
            ).required(false).value_parser(value_parser!(String))
        )
        .arg(
            arg!(
                -l --label <LABEL> "sets label location"
            ).required(false).value_parser(value_parser!(String))
        )
        .subcommand(
            Command::new("init")
                .about("initalize the directory")
        )
        .get_matches();



    // INIT
    if let Some(matches) = matches.subcommand_matches("init") {

        Config::write(&Config::new());

        DB::init(&config.db_location).await;

        fs::create_dir(Path::new(&config.label_location)).expect("Could not create label directory!");

        //create::init(&config.db_location).await;


        println!("INTITAILIZED DIRECTORY!");

        exit(0);
    }

    // GET CONFIG FILE
    if let Some(config_path) = matches.get_one::<PathBuf>("config"){

        if config_path.is_file() && config_path.try_exists().is_ok(){
            config = read_config(config_path.to_path_buf());
            println!("read config file")
        } else {
            eprintln!("config file does not exist!");
            exit(0);
        }

    } else {
        let file: PathBuf = PathBuf::from(DEFAULT_CONFIG_FILE);
        if file.is_file() && file.exists(){
            config = read_config(file);
            println!("read config file")
        } else {
            println!("no config file given");
        }
    }


    // GET PORT NUMBER
    if let Some(port_number) = matches.get_one::<i32>("port"){
        if *port_number > 0 && *port_number <= 65535 {
            config.port = *port_number as u16;
        } else {
            eprintln!("invalid port number!");
            exit(0);
        }
    }

    if let Some(host_address) = matches.get_one::<String>("address") {
        config.host_address = host_address.clone();
    }

    if let Some(db_location) = matches.get_one::<String>("db") {
        config.db_location = db_location.clone();
    }

    if let Some(label_location) = matches.get_one::<String>("label") {
        config.label_location = label_location.clone();
    }

    return config;
    
}