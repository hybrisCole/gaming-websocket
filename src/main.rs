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

use std::time::Instant;
use actix_web::actix::*;
use actix_web::server::HttpServer;
use actix_web::{middleware, ws, App, Error, HttpRequest, HttpResponse};

mod command;
mod chat_server;

pub struct WsSession {
  id: usize,
  /// Client must send ping at least once per 10 seconds, otherwise we drop
  /// connection.
  _hb: Instant,
  room: String,
  name: Option<String>,
}

impl Actor for WsSession {
  type Context = ws::WebsocketContext<Self, WsChatSessionState>;

  fn started(&mut self, ctx: &mut Self::Context) {
    println!("websocket sesssion started");
    info!("websocket sesssion started");
    let addr: Addr<_> = ctx.address();
    ctx.state()
      .addr
      .send(chat_server::Connect {
        addr: addr.recipient(),
      })
      .into_actor(self)
      .then(|res, act, ctx| {
        match res {
          Ok(res) => act.id = res,
          // something is wrong with chat server
          _ => ctx.stop(),
        }
        fut::ok(())
      })
      .wait(ctx);
  }
  fn stopped(&mut self, ctx: &mut Self::Context) {
    println!("websocket sesssion ended");
    info!("websocket sesssion ended");
    // notify chat server
    ctx.state().addr.do_send(chat_server::Disconnect { id: self.id });
  }
}

#[derive(Serialize, Deserialize)]
struct JoinChatPayload {
  room: String,
  name: String,
}

#[derive(Serialize, Deserialize)]
struct MessageChatPayload {
  message: String,
}

#[derive(Serialize, Deserialize)]
struct ListChatResponse {
  list: Vec<String>,
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<chat_server::MessageStruct> for WsSession {
  type Result = ();
  fn handle(&mut self, msg: chat_server::MessageStruct, ctx: &mut Self::Context) {
    ctx.text(msg.0);
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

        match cmd.command.as_ref() {
          "command:not_found" => println!("Received {:#?}", cmd),
          "command:chat:join" => {
            let payload: JoinChatPayload = serde_json::from_str(&cmd.payload).unwrap_or(JoinChatPayload {
              room: "Main".to_owned(),
              name: "anon".to_owned(),
            });
            self.room = payload.room;
            self.name =  Some(payload.name.to_owned());
            ctx.state().addr.do_send(chat_server::Join {
              id: self.id,
              name: self.room.clone(),
            });
            let message = serde_json::to_string(&chat_server::MessageResponse {
              message: format!("{} joined", payload.name.to_owned())
            }).unwrap();
            ctx.text(message);
          },
          "command:chat:list" => {
            ctx.state()
              .addr
              .send(chat_server::ListRooms)
              .into_actor(self)
              .then(|res, _, ctx| {
                match res {
                  Ok(rooms) => {
                    let chat_list = ListChatResponse{
                      list: rooms.to_owned()
                    };
                    let chat_list_response = serde_json::to_string(&chat_list).unwrap();
                    ctx.text(chat_list_response);
                  }
                  _ => println!("Something is wrong"),
                }
                fut::ok(())
              })
              .wait(ctx)
          },
          "command:chat:message" => {
            let payload: MessageChatPayload = serde_json::from_str(&cmd.payload).unwrap_or(MessageChatPayload {
              message: "msg".to_owned(),
            });
            ctx.state().addr.do_send(chat_server::ClientMessage {
              id: self.id,
              msg: payload.message,
              room: self.room.clone(),
            })
          },
          _ => ctx.text(format!("{{\"message\": \"Whoops! I can\'t understand you message {:?} \"}}", text)),
        }
      },
      ws::Message::Binary(_bin) => ctx.pong(&"Invalid message"),
      ws::Message::Close(_) => {
        ctx.stop();
      }
    }
  }
}

pub struct WsChatSessionState {
  addr: Addr<chat_server::ChatServer>,
}

/// Entry point for our route
fn chat_route(req: &HttpRequest<WsChatSessionState>) -> Result<HttpResponse, Error> {
  ws::start(
    req,
    WsSession {
      id: 0,
      _hb: Instant::now(),
      room: "Main".to_owned(),
      name: None,
    },
  )
}


fn main() {
  let _ = env_logger::init();
  let sys = System::new("game-socket");
  let server: Addr<_> = Arbiter::start(|_| chat_server::ChatServer::default());
  HttpServer::new(move || {
    let state = WsChatSessionState {
      addr: server.clone(),
    };
    App::with_state(state)
      .resource("/ws/", |r| r.route().f(chat_route))
      .middleware(middleware::Logger::default())
  }).bind("127.0.0.1:8080")
    .unwrap()
    .start();
  println!("Started http server: 127.0.0.1:8080");
  sys.run();
}