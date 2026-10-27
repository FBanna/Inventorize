use std::time::Duration;

use tokio::{task, time};

use crate::{config::config::Config, db::db::DB};



pub fn start_file_clean_up(db: &DB, config: &Config) -> tokio::task::JoinHandle<()> {

    
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(10));

        loop {
            interval.tick().await;
            run_clean_up(db, config).await;
        }
    });

    return forever

}

async fn run_clean_up(db: &DB, config: &Config) {



}