use std::fmt;

use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(PartialEq, Eq, Debug)]
pub enum Message {
    Shutdown,
    Ping(u32),
    Pong,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Shutdown => write!(f, "Shutdown"),
            Message::Ping(x) => write!(f, "Ping({})", x),
            Message::Pong => write!(f, "Pong"),
        }
    }
}

pub fn message_channel() -> (Sender<Message>, Receiver<Message>) {
    channel(10)
}
