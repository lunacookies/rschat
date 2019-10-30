use crate::Nick;
use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{fmt, net::SocketAddr};

#[derive(Clone, Deserialize, Serialize)]
pub struct Sender {
    pub nick: Nick,
    pub socket: SocketAddr,
}

impl fmt::Display for Sender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::*;

        write!(
            f,
            "{} {}",
            self.nick,
            "has joined the chat.".color("bright black").italic()
        )
    }
}

impl Sender {
    pub fn new() -> Self {
        Sender {
            nick: Nick::new(),
            socket: ([127, 0, 0, 1], 8080).into(),
        }
    }

    fn nick_taken(&self, senders: &mut Vec<Sender>) -> bool {
        senders
            .iter()
            .any(|sender| sender.nick.nick == self.nick.nick)
    }

    pub fn server_login(&self, senders: &mut Vec<Sender>) -> Result<SenderLoginResult> {
        let mut state = SenderLoginResult::Ok;

        if self.nick_taken(senders) {
            state = SenderLoginResult::NickTaken(self.nick.clone());
            warn!(target: "login_events", "Sender login attempt with taken nick {}", self.nick);
        } else {
            info!(
                target: "login_events",
                "Sender login at socket {} with nick {}", self.socket, self.nick
            );
            senders.push(self.clone());
        }

        crate::write_to_socket(&rmps::to_vec(&state)?, self.socket)?;

        Ok(state)
    }

    pub fn client_login(self, socket: SocketAddr) -> Result<SenderLoginResult> {
        use crate::Communication;
        use std::io::Read;
        use std::net::TcpListener;

        crate::write_to_socket(&rmps::to_vec(&Communication::Sender(self.clone()))?, socket)?;

        let listener = TcpListener::bind(self.socket)?;
        let mut login_result = SenderLoginResult::Ok;

        for stream in listener.incoming() {
            let mut stream = stream?;

            let mut msg = Vec::new();
            stream.read_to_end(&mut msg)?;

            match rmps::from_read_ref(&msg) {
                Ok(msg) => {
                    login_result = msg;
                    break;
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(login_result)
    }
}

#[derive(Deserialize, Serialize)]
pub enum SenderLoginResult {
    Ok,
    NickTaken(Nick),
}
