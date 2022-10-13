//! Data types

use serde::{Deserialize, Serialize};

/// Messages that the client sends to the watch
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
#[allow(non_camel_case_types)]
pub enum WatchMessage {
    find { n: bool },
}

/// Messages that the watch sends to the client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t")]
#[allow(non_camel_case_types)]
pub enum ClientMessage {
    music { n: MusicAction },
    lock_screen,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum MusicAction {
    play,
    pause,
    next,
    previous,
}
