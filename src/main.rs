use std::sync::{atomic::AtomicBool, Arc, Mutex};

use handler::handler;
use models::{ClientMessage, WatchMessage};
use nus::Client;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

mod handler;
mod models;
mod nus;

fn main() {
    let client: Client<ClientMessage, WatchMessage> = Client::new(handler).unwrap();
    let client_arc = Arc::new(Mutex::new(client));
    let handle = Client::run(client_arc.clone());

    let mut signals = Signals::new(&[SIGINT]).unwrap();

    for signal in &mut signals {
        match signal {
            SIGINT => {
                println!("gracefully exiting...");
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
