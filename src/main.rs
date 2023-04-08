use color_eyre::Result;
use text_io::read;

mod echo;
mod maelstrom;
use echo::echo_server::EchoServer;
use maelstrom::id::*;
use maelstrom::message::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut echo_server = EchoServer {
        id: None,
        msg_counter_id: 0,
    };

    loop {
        let line: String = read!("{}\n");
        let msg: Message = serde_json::from_str(&line)?;
        eprintln!("{}\n{:?}\n", line, msg);
        echo_server.handle_message(&msg)?;
    }
}