use crate::{config::config::Config, db::db::DB};



pub struct ServerState {

    pub db: DB,
    pub config: Config

}