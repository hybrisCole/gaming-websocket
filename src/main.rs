extern crate actix_web;
#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::actix::*;
use actix_web::server::HttpServer;
use actix_web::{middleware, ws, App, Error, HttpRequest, HttpResponse};

#[derive(Clone, Debug)]
pub struct Msg {
  inner: Vec<u32>,
}
impl Default for Msg {
  fn default() -> Msg {
    Msg {
      inner: vec![10; 10000],
    }
  }
}

pub struct WsSession;

impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self, ()>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    info!("websocket sesssion started");
  }

  fn stopped(&mut self, _ctx: &mut Self::Context) {
    info!("websocket sesssion ended");
  }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
  fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
    match msg {
      ws::Message::Ping(msg) => ctx.pong(&msg),
      ws::Message::Pong(msg) => ctx.ping(&msg),
      ws::Message::Text(text) => ctx.text(text),
      ws::Message::Binary(bin) => ctx.binary(bin),
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