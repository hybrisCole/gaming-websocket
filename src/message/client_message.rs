use actix_web::actix::Message;
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
    pub user: String,
}

impl Message for ClientMessage {
    type Result = ();
}
