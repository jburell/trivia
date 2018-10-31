include!("externs.rs");
use actix_web::actix as actix_server;

mod config;
mod res;
mod webserver;
mod relay;

use std::sync::{ Mutex, Arc };
use relay::Relay;
use webserver::WsServer;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode, Handshake};

struct Server {
    out: Sender,
}

impl Server {
    pub fn send(&mut self, msg: String) -> Result<()> {
        self.out.send(msg)
    }
}

impl Handler for Server {

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        self.send("Hello WebSocket".to_owned())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    /*let relay_actor = Arc::new(Mutex::new(Relay::new()));

    let sys = actix_server::System::new("Trivia Application");
    let server = WsServer::new(relay_actor.clone());
    server.start_server(&config::get_base_url());
    let _ = sys.run();*/


    listen("127.0.0.1:8080", |out| {
        Server { out: out }         
    }).unwrap()
}
