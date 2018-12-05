use actix_web::actix::Addr;
use chat_server;
pub struct WsChatSessionState {
    pub addr: Addr<chat_server::ChatServer>,
}
