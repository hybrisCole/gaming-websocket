//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.
extern crate serde_json;
use actix_web::actix::Message;
use actix_web::actix::*;
use rand::prelude::ThreadRng;
use rand::{self, Rng};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

pub struct MessageStruct(pub String);

impl Message for MessageStruct {
    type Result = ();
}

pub struct Connect {
    pub addr: Recipient<MessageStruct>,
}

impl Message for Connect {
    type Result = usize;
}

pub struct Disconnect {
    pub id: usize,
    pub name: String,
    pub user: String,
}

impl Message for Disconnect {
    type Result = ();
}

pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
    pub room: String,
    pub user: String,
}

impl Message for ClientMessage {
    type Result = ();
}

pub struct ListRooms;

impl Message for ListRooms {
    type Result = Vec<String>;
}

pub struct Join {
    pub id: usize,
    pub name: String,
    pub user: String,
}

impl Message for Join {
    type Result = ();
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<MessageStruct>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: RefCell<ThreadRng>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rng: RefCell::new(rand::thread_rng()),
            rooms,
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let message = serde_json::to_string(&MessageResponse {
                            message: message.to_owned(),
                        })
                        .unwrap();
                        let _ = addr.do_send(MessageStruct(message));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // notify all users in same room
        // register session with random id
        let id = self.rng.borrow_mut().gen::<usize>();
        self.sessions.insert(id, msg.addr);
        // auto join session to Main room
        self.rooms.get_mut(&"Main".to_owned()).unwrap().insert(id);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let Disconnect { id, name, user } = msg;
        let mut rooms: Vec<String> = Vec::new();
        // remove address
        if self.sessions.remove(&id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            if room == name {
                self.send_message(&room, &(user.clone() + " disconnected"), 0);
            }
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        let ClientMessage {
            id,
            msg,
            room,
            user,
        } = msg;
        self.send_message(&room, &(user + ": " + &msg), id);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name, user } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        self.send_message(&name, &(user + " connected"), id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}
