use anyhow::Result;
use std::net::SocketAddr;

fn main() -> Result<()> {
    simple_logger::init()?;

    let server_socket = rschat::parse_args()?;
    listen_for_msgs(server_socket)?;

    Ok(())
}

fn listen_for_msgs(socket: SocketAddr) -> Result<()> {
    use log::{debug, warn};
    use rschat::Communication;
    use std::io::Read;
    use std::net::TcpListener;

    let mut viewers = Vec::new();
    let mut senders = Vec::new();
    let listener = TcpListener::bind(socket)?;

    for stream in listener.incoming() {
        debug!("Received connection");
        let mut stream = stream?;

        let mut msg = Vec::new();
        stream.read_to_end(&mut msg)?;

        let communication: Communication = match rmps::from_read_ref(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to deserialise message: {}", e);
                continue;
            }
        };

        communication.handle(&mut viewers, &mut senders)?;
    }

    Ok(())
}
