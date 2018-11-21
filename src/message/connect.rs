use actix_web::actix::Message;
use actix_web::actix::*;
use message;
pub struct Connect {
  pub addr: Recipient<message::message_struct::MessageStruct>,
}

impl Message for Connect {
  type Result = usize;
}