use actix_web::actix::Message;

pub struct KeepAlive {
    pub id: usize,
    pub room: String,
    pub secs: u64,
}

impl Message for KeepAlive {
    type Result = ();
}
