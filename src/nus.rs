//! A thin wrapper around Daniel Thompson's [pynus](https://github.com/daniel-thompson/pynus/tree/5572b01b26e04c7f8c79e8407f4d202e7258bf6b).
//! This currently straights up calls wasptool, which is a python wrapper around pynus.
//!
//! It would be great to reimplement both pynus and wasptool in rust in the future.

use std::{
    collections::LinkedList,
    process::Command,
    str,
    sync::{Arc, Mutex},
    thread,
    thread::JoinHandle,
    time::Duration,
};

use rexpect::session::PtySession;

enum Cmd {
    Msg(String),
    Quit,
}

struct Client {
    msg_queue: LinkedList<Cmd>,
    session: PtySession,
}

impl Client {
    pub fn new() -> rexpect::errors::Result<Self> {
        // establish connection
        let mut session = rexpect::spawn("bin/pynus/pynus.py", Some(5_000))?;
        session.exp_regex(r#"Connected to [a-zA-Z0-9]* \([0-9A-F:]*\)\."#)?;
        session.exp_regex(r#"(Resolving services...)?"#)?;
        session.exp_regex(r#"Exit console using Ctrl-X\."#)?;
        std::thread::sleep(Duration::from_millis(500));

        // attempt to sync the tty
        session.send_line(&format!("{}", str::from_utf8(&[0x03]).unwrap()))?;
        session.exp_string(">>> ")?;

        Ok(Client {
            msg_queue: LinkedList::new(),
            session,
        })
    }

    fn send_cmd(&mut self, command: &str) -> rexpect::errors::Result<()> {
        let command_str = format!("from gadgetbridge import GB; GB({})", command);
        self.session.send_line(&command_str)?;

        // TODO parity check (if output is not equal to sent string, send it again)
        self.session.exp_string(&command_str)?;
        self.session.exp_string(">>> ")?;
        Ok(())
    }

    pub fn queue_cmd(this: Arc<Mutex<Self>>, command: Cmd) {
        let mut lock = this.lock().unwrap();
        lock.msg_queue.push_back(command);
    }

    fn parse_msg(&mut self) {}

    pub fn run(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut cur_line: Vec<char> = vec![];

            loop {
                // Read from REPL
                let mut lock = this.lock().unwrap();
                if let Some(c) = lock.session.try_read() {
                    if c == '\r' || c == '\n' {
                        println!("empyting... {:?}", cur_line);
                        cur_line.clear();
                    } else {
                        cur_line.push(c);
                        println!("line {:?}", cur_line);
                    }
                }

                // send any requested messages
                if !lock.msg_queue.is_empty() {
                    let msg = lock.msg_queue.pop_front().unwrap();
                    match msg {
                        Cmd::Msg(msg) => {
                            lock.send_cmd(&msg).unwrap();
                        },
                        Cmd::Quit => {
                            lock.session
                                .send_line(str::from_utf8(&[0x18]).unwrap())
                                .unwrap();
                            return;
                        },
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
        time::Duration,
    };

    use super::{Client, Cmd};

    // #[test]
    // fn create_client() {
    //     Client::new().unwrap();
    // }

    // #[test]
    // fn client_cmd() {
    //     let mut client = Client::new().unwrap();
    //     client.cmd(r#"{"t": "find", "n": false}"#).unwrap();
    // }

    #[test]
    fn listen() {
        let client = Client::new().unwrap();
        let client_arc = Arc::new(Mutex::new(client));
        let handle = Client::run(client_arc.clone());

        thread::sleep(Duration::from_secs(2));
        Client::queue_cmd(
            client_arc.clone(),
            Cmd::Msg(r#"{"t": "find", "n": true}"#.into()),
        );

        thread::sleep(Duration::from_secs(2));
        Client::queue_cmd(
            client_arc.clone(),
            Cmd::Msg(r#"{"t": "find", "n": false}"#.into()),
        );

        thread::sleep(Duration::from_secs(2));
        Client::queue_cmd(client_arc.clone(), Cmd::Quit);

        handle.join().unwrap();
    }
}
