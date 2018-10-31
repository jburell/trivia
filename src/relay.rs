use actix::prelude::*;
use std::time::{Instant, Duration};
use ::res::websocket::MyWebSocket;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Relay {
    reciever_addr: Option<Addr<MyWebSocket>>,
}

impl Relay {
    pub fn new() -> Self {
        Relay {
            reciever_addr: None,
        }
    }

    pub fn send_message(&mut self) {
    }

    pub fn set_reciever_addr(&mut self, addr: Addr<MyWebSocket>) {
        self.reciever_addr = Some(addr);
    }
}

impl Actor for Relay {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }
}
