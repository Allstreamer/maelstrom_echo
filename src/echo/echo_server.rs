use crate::maelstrom::message::Body;
use crate::maelstrom::message::BodyType;
use crate::Message;
use crate::ID;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
pub struct EchoServer {
    pub id: Option<ID>,
    pub msg_counter_id: u64,
}

impl EchoServer {
    pub fn handle_message(&mut self, msg: &Message) -> Result<()> {
        let response = match msg.body.body_type {
            BodyType::Init => self.handle_init(msg)?,
            BodyType::Echo => self.handle_echo(msg)?,
            _ => Message {
                src: self.id.context("Client hot been initilized!")?,
                dest: msg.src,
                body: Body {
                    body_type: BodyType::Error,
                    msg_id: inc_and_return(&mut self.msg_counter_id),
                    in_reply_to: msg.body.msg_id,
                    code: Some(10),
                    text: Some("".to_string()),
                    ..Default::default()
                },
            },
        };
        println!("{}", serde_json::to_string(&response)?);
        Ok(())
    }

    fn handle_init(&mut self, msg: &Message) -> Result<Message> {
        self.id = Some(msg.dest);

        let mut response = Message {
            src: msg.dest, // Use msg.dest here just to avoid unwraping self.id
            dest: msg.src,
            body: Body {
                body_type: BodyType::InitOk,
                in_reply_to: msg.body.msg_id,
                ..Default::default()
            },
        };
        response.body.msg_id = inc_and_return(&mut self.msg_counter_id);
        Ok(response)
    }

    fn handle_echo(&mut self, msg: &Message) -> Result<Message> {
        let mut response = Message {
            src: self.id.context("Not Initlized")?,
            dest: msg.src,
            body: Body {
                body_type: BodyType::EchoOk,
                in_reply_to: msg.body.msg_id,
                echo: msg.body.echo.clone(),
                ..Default::default()
            },
        };
        response.body.msg_id = inc_and_return(&mut self.msg_counter_id);
        Ok(response)
    }
}

fn inc_and_return(int: &mut u64) -> Option<u64> {
    *int += 1;
    Some(*int)
}
