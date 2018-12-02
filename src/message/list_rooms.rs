use actix_web::actix::Message;

pub struct ListRooms;

impl Message for ListRooms {
    type Result = Vec<String>;
}
