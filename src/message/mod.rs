pub mod client_message;
pub mod command;
pub mod connect;
pub mod disconnect;
pub mod join;
pub mod keep_alive;
pub mod message_struct;
pub mod session_state;

#[derive(Serialize, Deserialize)]
pub struct JoinChatPayload {
    pub room: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageChatPayload {
    pub message: String,
}
