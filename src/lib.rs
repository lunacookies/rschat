use anyhow::Result;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod communication;
mod message;
mod nick;
mod sender;
mod viewer;

pub use communication::Communication;
pub use message::Message;
pub use nick::Nick;
pub use sender::{Sender, SenderLoginResult};
pub use viewer::Viewer;

pub fn prompt(p: &str) -> Result<String> {
    use std::io::{self, Write};

    print!("{}: ", p);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn parse_args() -> Result<SocketAddr> {
    use anyhow::anyhow;
    use std::env;

    let err_msg = "did not receive server socket argument";

    let args = env::args().skip(1).next().ok_or(anyhow!(err_msg))?;

    Ok(args.parse()?)
}

pub fn gen_rand_port() -> u16 {
    use rand::Rng;

    // We add 1_024 because all ports below that are privileged and are not to be used for
    // applications. Additionally, the maximum possible port number is 65_535.
    rand::thread_rng().gen_range(1_024, 65_535)
}

pub fn get_local_ip() -> Result<Ipv4Addr> {
    use anyhow::anyhow;
    use get_if_addrs::IfAddr;

    let err_msg = "failed to get local IP address";

    if let IfAddr::V4(ipv4) = &get_if_addrs::get_if_addrs()?
        .iter()
        .filter(|interface| interface.name == "en0")
        .next()
        .ok_or(anyhow!(err_msg))?
        .addr
    {
        Ok(ipv4.ip)
    } else {
        Err(anyhow!(err_msg))
    }
}

pub fn gen_rand_local_socket() -> Result<SocketAddr> {
    let addr = IpAddr::V4(get_local_ip()?);
    Ok(SocketAddr::new(addr, gen_rand_port()))
}

pub fn write_to_socket(buf: &[u8], socket: SocketAddr) -> Result<()> {
    use std::io::Write;
    use std::net::TcpStream;

    let mut stream = TcpStream::connect(socket)?;
    stream.write_all(buf)?;

    Ok(())
}

fn colour_on_hash<T: Hash>(s: &T) -> &'static str {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();

    // We limit the possible colours to _not_ include black and white, as those don’t provide enough
    // differentiation.
    let colour = hash % 6 + 1;

    match colour {
        1 => "red",
        2 => "green",
        3 => "yellow",
        4 => "blue",
        5 => "magenta",
        6 => "cyan",
        _ => panic!(), // This is impossible due to earlier modulo
    }
}
