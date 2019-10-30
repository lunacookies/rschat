use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::{fmt, net::SocketAddr};

#[derive(Deserialize, Serialize)]
pub struct Viewer {
    pub socket: SocketAddr,
    colour: String,
}

impl fmt::Display for Viewer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::*;

        write!(f, "{}", self.socket.to_string().color(self.colour.clone()))
    }
}

impl Viewer {
    pub fn new() -> Self {
        Viewer {
            socket: ([0, 0, 0, 0], 8080).into(),
            colour: String::new(),
        }
    }

    pub fn recalculate_colour(&mut self) {
        self.colour = crate::colour_on_hash(&self.socket).into();
    }

    pub fn server_login(self, viewers: &mut Vec<Viewer>) {
        info!(target: "login_events", "Viewer login at {}", self);
        viewers.push(self);
    }

    pub fn client_login(self, socket: SocketAddr) -> Result<()> {
        use crate::Communication;

        println!("Logging in...");
        crate::write_to_socket(&rmps::to_vec(&Communication::Viewer(self))?, socket)?;
        println!("Logged in!");
        Ok(())
    }
}
