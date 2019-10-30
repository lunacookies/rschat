use crate::{Message, Sender, Viewer};
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub enum Communication {
    Message(Message),
    Sender(Sender),
    Viewer(Viewer),
}

impl fmt::Display for Communication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Communication::Message(msg) => write!(f, "{}", msg),
            Communication::Sender(sender) => write!(f, "{}", sender),
            Communication::Viewer(_) => unimplemented!(),
        }
    }
}

impl Communication {
    fn forward(&self, viewer: &Viewer) -> Result<()> {
        crate::write_to_socket(&rmps::to_vec(self)?, viewer.socket)
    }

    pub fn handle(self, viewers: &mut Vec<Viewer>, senders: &mut Vec<Sender>) -> Result<()> {
        use crate::SenderLoginResult;

        match self {
            Communication::Viewer(viewer) => viewer.server_login(viewers),
            Communication::Sender(sender) => {
                let sender_login_result = sender.server_login(senders)?;

                if let SenderLoginResult::Ok = sender_login_result {
                    viewers.iter().for_each(|viewer| {
                        info!(target: "login_events", "Forwarding sender login to {}", &viewer);
                        let _ = Communication::Sender(sender.clone()).forward(viewer);
                    });
                }
            }
            Communication::Message(msg) => {
                info!(
                    target: "msg_events",
                    "Received message ‘{}’ from nick {}", msg.body, msg.origin.nick
                );

                for viewer in viewers {
                    info!(target: "msg_events", "Forwarding message to {}", &viewer);
                    let _ = Communication::Message(msg.clone()).forward(viewer);
                }
            }
        }

        Ok(())
    }
}
