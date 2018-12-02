use actix_web::actix::Message;
pub struct Disconnect {
    pub id: usize,
    pub name: String,
    pub user: String,
}

impl Message for Disconnect {
    type Result = ();
}
