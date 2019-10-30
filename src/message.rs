use crate::Sender;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt, net::SocketAddr};

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    pub origin: Sender,
    pub body: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.origin.nick, self.body)
    }
}

impl Message {
    pub fn send(self, socket: SocketAddr) -> Result<()> {
        use crate::Communication;
        crate::write_to_socket(&rmps::to_vec(&Communication::Message(self))?, socket)
    }
}
