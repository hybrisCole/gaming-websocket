use crate::chat_server;
use actix_web::actix::Addr;
use actix_web::actix::Message;
use actix_web::actix::*;

pub struct WsChatSessionState {
    pub addr: Addr<chat_server::ChatServer>,
}

pub struct KeepAlive {
    pub id: usize,
    pub room: String,
    pub secs: u64,
}

impl Message for KeepAlive {
    type Result = ();
}

pub struct Join {
    pub id: usize,
    pub name: String,
    pub user: String,
}

impl Message for Join {
    type Result = ();
}

pub struct Disconnect {
    pub id: usize,
    pub name: String,
    pub user: String,
}

impl Message for Disconnect {
    type Result = ();
}
pub struct Connect {
    pub addr: Recipient<MessageStruct>,
}

pub struct MessageStruct(pub String);

impl Message for MessageStruct {
    type Result = ();
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

impl Message for Connect {
    type Result = usize;
}

pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
    pub user: String,
}

impl Message for ClientMessage {
    type Result = ();
}

#[derive(Serialize, Deserialize)]
pub struct JoinChatPayload {
    pub room: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageChatPayload {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub payload: String,
}
