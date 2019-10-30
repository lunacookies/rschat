use anyhow::Result;
use std::net::SocketAddr;

fn main() -> Result<()> {
    use rschat::Viewer;

    let socket = rschat::gen_rand_local_socket()?;
    let mut me = Viewer::new();
    me.socket = socket;
    me.recalculate_colour();

    let server_socket = rschat::parse_args()?;
    me.client_login(server_socket)?;

    listen(socket)?;

    Ok(())
}

fn listen(socket: SocketAddr) -> Result<()> {
    use rschat::Communication;
    use std::{io::Read, net::TcpListener};

    let listener = TcpListener::bind(socket)?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut msg = Vec::new();
        stream.read_to_end(&mut msg)?;

        let msg: Communication = match rmps::from_read_ref(&msg) {
            Ok(msg) => msg,
            _ => continue,
        };

        println!("{}", msg);
    }

    Ok(())
}
