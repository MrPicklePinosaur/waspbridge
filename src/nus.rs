//! A thin wrapper around Daniel Thompson's [pynus](https://github.com/daniel-thompson/pynus/tree/5572b01b26e04c7f8c79e8407f4d202e7258bf6b).
//! This currently straights up calls wasptool, which is a python wrapper around pynus.
//!
//! It would be great to reimplement both pynus and wasptool in rust in the future.

use std::str;
use std::{process::Command, time::Duration};

use rexpect::session::PtySession;

struct Client {
    session: PtySession
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

        Ok(Client { session })
    }

    pub fn cmd(&mut self, command: &str) -> rexpect::errors::Result<()> {
        let command_str = format!("from gadgetbridge import GB; GB({})", command);
        self.session.send_line(&command_str)?;
        self.session.exp_string(&command_str)?;
        self.session.exp_string(">>> ")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Client;

    // #[test]
    // fn create_client() {
    //     Client::new().unwrap();
    // }

    #[test]
    fn client_cmd() {
        let mut client = Client::new().unwrap();
        client.cmd(r#"{"t": "find", "n": false}"#).unwrap();
    }

}
