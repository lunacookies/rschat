use anyhow::Result;
use rschat::Sender;
use std::net::SocketAddr;

fn main() -> Result<()> {
    use rschat::Message;

    let server_socket = rschat::parse_args()?;
    let me = login(server_socket)?;

    loop {
        let msg = Message {
            origin: me.clone(),
            body: prompt_for_msg()?,
        };
        msg.send(server_socket)?;
    }
}

fn prompt_for_nick() -> Result<String> {
    rschat::prompt("nick")
}

fn prompt_for_msg() -> Result<String> {
    rschat::prompt("message")
}

fn login(server_socket: SocketAddr) -> Result<Sender> {
    use rschat::SenderLoginResult;

    println!("Logging in...");

    let mut me = Sender::new();
    me.socket = rschat::gen_rand_local_socket()?;

    loop {
        let nick = prompt_for_nick()?;
        me.nick.nick = nick;
        me.nick.recalculate_colour();

        let login_result = me.clone().client_login(server_socket)?;

        if let SenderLoginResult::Ok = login_result {
            break;
        } else {
            println!("Nick {} already taken -- try a different one.", me.nick);
        }
    }

    println!("Logged in!");

    Ok(me)
}
