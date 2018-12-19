#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate byteorder;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;

mod chat_server;
mod message;
mod session;

use actix_web::actix::*;
use actix_web::server::HttpServer;
use actix_web::{middleware, ws, App, Error, HttpRequest, HttpResponse};

use crate::message::WsChatSessionState;

use crate::session::WsSession;

fn chat_route(req: &HttpRequest<WsChatSessionState>) -> Result<HttpResponse, Error> {
    ws::start(
        req,
        WsSession {
            id: 0,
            room: "Main".to_owned(),
            name: "QAnon".to_owned(),
        },
    )
}

fn main() {
    // let socket_url = "192.168.1.2:8080";
    let socket_url = "127.0.0.1:8080";
    env_logger::init();
    let sys = System::new("game-socket");
    let server: Addr<_> = Arbiter::start(|_| chat_server::ChatServer::default());
    HttpServer::new(move || {
        let state = WsChatSessionState {
            addr: server.clone(),
        };
        App::with_state(state)
            .resource("/ws/", |r| r.route().f(chat_route))
            .middleware(middleware::Logger::default())
    })
    .bind(socket_url)
    .unwrap()
    .start();
    println!(
        "{}",
        ["StartedInstant::now http server: ", socket_url].join(" ")
    );
    sys.run();
}
