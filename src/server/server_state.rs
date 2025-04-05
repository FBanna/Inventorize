use crate::{cli::config::Config, db::components::Components};



pub struct ServerState {

    pub db: Components,
    pub config: Config

}