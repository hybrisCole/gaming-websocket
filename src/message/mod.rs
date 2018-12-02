pub mod client_message;
pub mod connect;
pub mod disconnect;
pub mod join;
pub mod list_rooms;
pub mod message_struct;
pub mod session_state;
pub mod command;

#[derive(Serialize, Deserialize)]
pub struct JoinChatPayload {
  pub room: String,
  pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageChatPayload {
  pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListChatResponse {
  pub list: Vec<String>,
}