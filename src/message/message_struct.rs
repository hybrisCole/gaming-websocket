use actix_web::actix::Message;
pub struct MessageStruct(pub String);

impl Message for MessageStruct {
  type Result = ();
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
  pub message: String,
}