use anyhow::Result;
use async_std::{net::TcpStream, prelude::*};
use rschat::{Sender, Viewer};
use std::net::SocketAddr;

#[async_std::main]
async fn main() -> Result<()> {
    simple_logger::init()?;

    let server_socket = rschat::parse_args()?;
    listen_for_msgs(server_socket).await?;

    Ok(())
}

async fn listen_for_msgs(socket: SocketAddr) -> Result<()> {
    use async_std::{
        net::TcpListener,
        sync::{Arc, Mutex},
        task,
    };

    let viewers = Arc::new(Mutex::new(Vec::new()));
    let senders = Arc::new(Mutex::new(Vec::new()));

    let listener = TcpListener::bind(socket).await?;
    let mut incoming = listener.incoming();

    let mut tasks = Vec::new();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;

        let viewers = viewers.clone();
        let senders = senders.clone();

        tasks.push(task::spawn(async move {
            let mut viewers = viewers.lock().await;
            let mut senders = senders.lock().await;

            match handle_connection(stream, &mut *viewers, &mut *senders).await {
                Err(e) => eprintln!("{}", e),
                _ => (),
            };
        }));
    }

    for task in tasks {
        task.await;
    }

    Ok(())
}

async fn handle_connection(
    mut stream: TcpStream,
    mut viewers: &mut Vec<Viewer>,
    mut senders: &mut Vec<Sender>,
) -> Result<()> {
    use anyhow::anyhow;
    use log::debug;
    use rschat::Communication;

    debug!("Received connection");

    let mut msg = Vec::new();
    stream.read_to_end(&mut msg).await?;

    let communication: Communication = match rmps::from_read_ref(&msg) {
        Ok(msg) => msg,
        Err(e) => {
            return Err(anyhow!("Error deserialising message: {}", e));
        }
    };

    communication.handle(&mut viewers, &mut senders)?;

    Ok(())
}
