use actix_web::actix::Addr;
use actix_web::actix::*;
use actix_web::ws;
use message::client_message::ClientMessage;
use message::command::Command;
use message::connect::Connect;
use message::disconnect::Disconnect;
use message::join::Join;
use message::keep_alive::KeepAlive;
use message::message_struct::MessageResponse;
use message::message_struct::MessageStruct;
use message::session_state::WsChatSessionState;
use message::{JoinChatPayload, MessageChatPayload};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WsSession {
    pub id: usize,
    pub room: String,
    pub name: String,
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<MessageStruct> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: MessageStruct, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self, WsChatSessionState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("websocket sesssion started");
        info!("websocket sesssion started");
        let addr: Addr<_> = ctx.address();
        ctx.state()
            .addr
            .send(Connect {
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
        ctx.state().addr.do_send(Disconnect {
            id: self.id,
            name: self.room.clone(),
            user: self.name.clone(),
        });
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Pong(msg) => ctx.ping(&msg),
            ws::Message::Text(text) => {
                let cmd: Command = serde_json::from_str(&text).unwrap_or(Command {
                    command: "command:not_found".to_owned(),
                    payload: "".to_owned(),
                });

                match cmd.command.as_ref() {
                    "command:not_found" => println!("Received {:#?}", cmd),
                    "command:chat:join" => {
                        let payload: JoinChatPayload = serde_json::from_str(&cmd.payload)
                            .unwrap_or(JoinChatPayload {
                                room: "Main".to_owned(),
                                name: "anon".to_owned(),
                            });
                        self.room = payload.room;
                        self.name = payload.name;
                        ctx.state().addr.do_send(Join {
                            id: self.id,
                            name: self.room.clone(),
                            user: self.name.clone(),
                        });
                        let message = serde_json::to_string(&MessageResponse {
                            message: format!("{} joined", self.name.clone()),
                        })
                        .unwrap();
                        ctx.text(message);
                    }
                    "command:chat:message" => {
                        let payload: MessageChatPayload = serde_json::from_str(&cmd.payload)
                            .unwrap_or(MessageChatPayload {
                                message: "msg".to_owned(),
                            });
                        ctx.state().addr.do_send(ClientMessage {
                            id: self.id,
                            msg: payload.message,
                            room: self.room.clone(),
                            user: self.name.clone(),
                        })
                    }
                    "command:keepAlive" => {
                        let start = SystemTime::now();
                        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Duh");
                        ctx.state().addr.do_send(KeepAlive {
                            id: self.id,
                            room: self.room.clone(),
                            secs: since_the_epoch.as_secs(),
                        });
                    }
                    _ => ctx.text(format!(
                        "{{\"message\": \"Whoops! I can\'t understand you message {:?} \"}}",
                        text
                    )),
                }
            }
            ws::Message::Binary(_bin) => ctx.pong(&"Invalid message"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}
