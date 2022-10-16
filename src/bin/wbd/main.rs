use std::{
    io::Read,
    os::unix::net::{UnixListener, UnixStream},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread,
};

use log::{error, info};
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

    thread::spawn(|| {
        listen();
    });

    handle.join().unwrap();
}

fn listen() -> std::io::Result<()> {
    const SOCKET_PATH: &'static str = "/tmp/waspbridge";

    // listen on unix socket
    let listener = UnixListener::bind(SOCKET_PATH)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        thread::spawn(move || {
            let mut str_buf = String::new();

            loop {
                stream.read_to_string(&mut str_buf);
            }
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::listen;

    #[test]
    fn listen_test() {
        let res = listen().unwrap();
    }
}
