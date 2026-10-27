use std::{sync::Arc, time::Duration};

use sqlx::{Pool, Postgres};
use tokio::{task, time};

use crate::{config::config::Config, db::{component::properties::file::{file::clean_up_file, service::ComponentFileService}, db::DB}};



pub fn start_file_clean_up(config: Config, db: DB) -> tokio::task::JoinHandle<()> {

    
    let forever: task::JoinHandle<()>  = task::spawn(async move {

        let mut interval = time::interval(Duration::from_mins(config.clean_up_interval as u64));

        loop {
            interval.tick().await;
            run_clean_up(&db, &config).await;
        }

    });

    return forever

}

async fn run_clean_up(db: &DB, config: &Config) {


    let files = db.get_all_file_clean_up_queue_id().await.unwrap();

    for file in files {
        println!("trying to delete");
        let result = clean_up_file(file.file_id, &config).await;

        if result.is_ok() {
            println!("success");
            db.remove_id_from_file_clean_up_queue(file.file_id).await.unwrap();

        }
    }


}