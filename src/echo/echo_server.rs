// This file is part of Maelstrom Echo which is released under GNU GPL v2.0.
// See file LICENSE.

use crate::maelstrom::message::Body;
use crate::maelstrom::message::BodyType;
use crate::Message;
use crate::ID;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use rand::Rng;

pub struct EchoServer {
    pub id: Option<ID>,
    pub msg_counter_id: u64,
    pub rng: rand::rngs::ThreadRng,
    // Part of Broadcast workload
    pub msg_state: Vec<u32>,
    pub node_ids: Vec<ID>,
}

impl EchoServer {
    pub fn new() -> Self {
        Self {
            id: None,
            msg_counter_id: 0,
            rng: rand::thread_rng(),
            msg_state: vec![],
            node_ids: vec![],
        }
    }

    pub fn handle_message(&mut self, msg: &Message) -> Result<()> {
        let mut response = match msg.body.body_type {
            BodyType::Init => self.handle_init(msg)?,
            BodyType::Echo => self.handle_echo(msg)?,
            BodyType::Generate => self.handle_generate(msg)?,
            BodyType::Read => self.handle_read(msg)?,
            BodyType::Broadcast => self.handle_broadcast(msg)?,
            BodyType::BroadcastOk => return Ok(()),
            BodyType::Topology => self.handle_topology(msg)?,
            _ => Message {
                src: self.id.context("Client hot been initilized!")?,
                dest: msg.src,
                body: Body {
                    body_type: BodyType::Error,
                    msg_id: Self::inc_and_return(&mut self.msg_counter_id),
                    code: Some(13),
                    text: Some("".to_string()),
                    ..Default::default()
                },
            },
        };
        response.body.in_reply_to = msg.body.msg_id;
        println!("{}", serde_json::to_string(&response)?);
        Ok(())
    }

    fn inc_and_respond(&mut self, mut response: Message) -> Message {
        Self::inc_and_return(&mut self.msg_counter_id);
        response.body.msg_id =
            Self::inc_and_return(&mut self.msg_counter_id);
        response
    }

    fn inc_and_return(int: &mut u64) -> Option<u64> {
        *int += 1;
        Some(*int)
    }

    fn create_response(
        &self,
        msg: &Message,
        body: Body,
    ) -> Result<Message> {
        Ok(Message {
            src: self.id.context("Not Initlized")?,
            dest: msg.src,
            body,
        })
    }

    fn handle_init(&mut self, msg: &Message) -> Result<Message> {
        self.id = Some(msg.dest);
        self.node_ids = msg
            .body
            .node_ids
            .clone()
            .context("I")?
            .iter()
            .filter(|x| **x != msg.dest)
            .map(|x| x.to_owned())
            .collect::<Vec<ID>>();
        // eprintln!("{:?},\n{:?}", self.id, self.node_ids);
        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::InitOk,
                ..Default::default()
            },
        )?;

        Ok(self.inc_and_respond(response))
    }

    fn handle_echo(&mut self, msg: &Message) -> Result<Message> {
        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::EchoOk,
                echo: msg.body.echo.clone(),
                ..Default::default()
            },
        )?;
        Ok(self.inc_and_respond(response))
    }

    fn handle_generate(&mut self, msg: &Message) -> Result<Message> {
        let random_id = self.rng.gen::<u128>().to_string();

        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::GenerateOk,
                id: Some(random_id),
                ..Default::default()
            },
        )?;
        Ok(self.inc_and_respond(response))
    }

    fn handle_read(&mut self, msg: &Message) -> Result<Message> {
        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::ReadOk,
                messages: Some(self.msg_state.clone()),
                ..Default::default()
            },
        )?;
        Ok(self.inc_and_respond(response))
    }
    fn handle_broadcast(&mut self, msg: &Message) -> Result<Message> {
        self.msg_state.push(
            msg.body
                .message
                .context("Broadcast didn't supply Message")?,
        );

        // Only Distribute when message comes from client
        if let ID::Client(_v) = msg.src {
            for node in &self.node_ids {
                println!(
                    "{}",
                    serde_json::to_string(&Message {
                        src: self.id.context("Not initlized")?,
                        dest: *node,
                        body: Body {
                            body_type: BodyType::Broadcast,
                            message: msg.body.message,
                            ..Default::default()
                        }
                    })?
                );
            }
        }

        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::BroadcastOk,
                ..Default::default()
            },
        )?;
        Ok(self.inc_and_respond(response))
    }
    fn handle_topology(&mut self, msg: &Message) -> Result<Message> {
        let response = self.create_response(
            msg,
            Body {
                body_type: BodyType::TopologyOk,
                ..Default::default()
            },
        )?;
        Ok(self.inc_and_respond(response))
    }
}
