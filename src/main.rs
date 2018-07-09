#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate actix_web;
extern crate env_logger;
extern crate rand;
extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use actix_web::actix::*;
use actix_web::server::HttpServer;
use actix_web::{middleware, ws, App, Error, HttpRequest, HttpResponse};

mod command;
mod chat_server;

pub struct WsSession;

impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self, ()>;
  fn started(&mut self, _ctx: &mut Self::Context) {
    println!("websocket sesssion started");
    info!("websocket sesssion started");
  }
  fn stopped(&mut self, _ctx: &mut Self::Context) {

    println!("websocket sesssion ended");
    info!("websocket sesssion ended");
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    match msg {
      ws::Message::Ping(msg) => ctx.pong(&msg),
      ws::Message::Pong(msg) => ctx.ping(&msg),
      ws::Message::Text(text) => {
        let cmd: command::Command = serde_json::from_str(&text)
          .unwrap_or(command::Command { command: "command:not_found".to_owned(), payload:"".to_owned() });
        println!("Received {:#?}", cmd);
        ctx.text(serde_json::to_string(&cmd).unwrap())
      },
      ws::Message::Binary(_bin) => ctx.pong(&"Invalid message"),
      ws::Message::Close(_) => {
        ctx.stop();
      }
    }
  }
}

pub fn ws_handler(r: &HttpRequest) -> Result<HttpResponse, Error> {
  ws::start(r, WsSession)
}
fn main() {
  ::std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  let sys = System::new("game-socket");
  HttpServer::new(move || App::new()
    .middleware(middleware::Logger::default())
    .resource("/ws/", |r| r.route().f(ws_handler)))
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();
  let _ = sys.run();
}