
use std::{io::Error, sync::{Arc, atomic::AtomicBool}};

use crate::config::config::Config;
use db::{components::ComponentServices, db::DB};
use signal_hook::{consts::TERM_SIGNALS, flag};


// use signal_hook::consts::signal::*;

mod server;
mod config;
mod db;
mod label;
mod error;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let config: Config = config::command::get_config().await;

    let component_db = DB::init(&config.db_location).await;

    // // Make sure double CTRL+C and similar kills
    // let term_now = Arc::new(AtomicBool::new(false));
    // for sig in TERM_SIGNALS {
    //     // When terminated by a second term signal, exit with exit code 1.
    //     // This will do nothing the first time (because term_now is false).
    //     flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
    //     // But this will "arm" the above for the second time, by setting it to true.
    //     // The order of registering these is important, if you put this one first, it will
    //     // first arm and then terminate ‒ all in the first round.
    //     flag::register(*sig, Arc::clone(&term_now))?;
    // }

    // // Subscribe to all these signals with information about where they come from. We use the
    // // extra info only for logging in this example (it is not available on all the OSes or at
    // // all the occasions anyway, it may return `Unknown`).
    // let mut sigs = vec![
    //     // Some terminal handling
    //     SIGTSTP, SIGCONT, SIGWINCH,
    //     // Reload of configuration for daemons ‒ um, is this example for a TUI app or a daemon
    //     // O:-)? You choose...
    //     SIGHUP,
    //     // Application-specific action, to print some statistics.
    //     SIGUSR1,
    // ];
    // sigs.extend(TERM_SIGNALS);
    // let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;



    

    let handle = server::entry::start_server(config, component_db).await;



    handle.await.unwrap();

    Ok(())

    // handle.close();
    // signals_task.await?;


}

// async fn handle_signals(mut signals: Signals) {
//     while let Some(signal) = signals.next().await{
//         match signal {
//             SIGTERM => {

//             }
//         }
//     }
// }





    // let component = db::components::Component{
    //     //ID:5000,
    //     id: None,
    //     name:("Resistor".to_string()),
    //     size:Some("0402".to_string()),
    //     value:Some("60 OHM".to_string()),
    //     info:None,
    //     stock:5000,
    //     origin:None, 
    //     //url: None,
    //     label: Some("vial".to_string()),
    //     image: false,
    //     datasheet: false
    // };

    // println!("start");

    // for i in 1..100 {
    //     component_db.add(&component).await;
        
    // }

    // println!("stop");



    

    // //component.build();

    // //label::label::Label::new(component).build();

    // component_db.add(&component).await;

    // let component = db::components::Component{
    //     //ID:5000,
    //     id: None,
    //     name:("Resistor".to_string()),
    //     size:Some("0603".to_string()),
    //     value:Some("180 OHM".to_string()),
    //     info:Some("Thick Film".to_string()),
    //     stock:10,
    //     origin:None, 
    //     //url: None,
    //     label: None,
    //     image: false,
    //     datasheet: false
    // };

    // component_db.add(&component).await;