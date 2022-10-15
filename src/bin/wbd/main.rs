use std::sync::{atomic::AtomicBool, Arc, Mutex};

use log::info;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use waspbridge::{
    handler::handler,
    models::{ClientMessage, WatchMessage},
    nus::Client,
};

fn main() {
    env_logger::builder().init();

    let client: Client<ClientMessage, WatchMessage> = Client::new(handler).unwrap();
    let client_arc = Arc::new(Mutex::new(client));
    let handle = Client::run(client_arc.clone());

    let mut signals = Signals::new(&[SIGINT]).unwrap();

    for signal in &mut signals {
        match signal {
            SIGINT => {
                info!("gracefully exiting...");
                Client::send_quit(client_arc.clone());
                break;
            },
            _ => {
                unreachable!()
            },
        }
    }

    handle.join().unwrap();
}
