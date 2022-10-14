//! Command handler

use std::process::Command;

use crate::models::{ClientMessage, MusicAction};

pub fn handler(msg: String) -> anyhow::Result<()> {
    println!("got message {}", msg);

    let client_msg: ClientMessage = serde_json::from_str(&msg)?;
    println!("{:?}", client_msg);

    match client_msg {
        ClientMessage::music { n } => handle_music(n)?,
        ClientMessage::lock_screen => handle_lockscreen()?,
        _ => {},
    }

    Ok(())
}

fn handle_music(action: MusicAction) -> anyhow::Result<()> {
    let mut cmd = Command::new("mpc");
    match action {
        MusicAction::play => cmd.arg("play"),
        MusicAction::pause => cmd.arg("pause"),
        MusicAction::next => cmd.arg("next"),
        MusicAction::previous => cmd.arg("prev"),
        MusicAction::volumeup => return Ok(()),
        MusicAction::volumedown => return Ok(()),
    };
    cmd.output()?;

    Ok(())
}

fn handle_lockscreen() -> anyhow::Result<()> {
    Command::new("slock").output()?;
    Ok(())
}
