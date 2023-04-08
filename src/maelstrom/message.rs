use serde::{Deserialize, Serialize};

use super::id::ID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: ID,
    pub dest: ID,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BodyType {
    Init,
    InitOk,
    Echo,
    EchoOk,
    Error,
}
impl Default for BodyType {
    fn default() -> Self {
        Self::Error
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Body {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub body_type: BodyType,
    /// Unique Id of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<u64>,
    /// Id of the message this is a reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<u64>,
    /// Error: Error Code Used:(0-999) Open:(1000-Infinty)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u32>,
    /// Error: Err Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Init: ID of the client that is being Initilized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<ID>,
    /// Init: IDs of other Nodes/Clients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<ID>>,
    /// Echo: Echo Message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: ID,
    pub known_nodes: Vec<ID>,
}
