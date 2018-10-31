include!("externs.rs");

mod config;
mod relay;
mod res;
mod webserver;

use actix_web::{
    fs, http, http::{StatusCode}, middleware, middleware::cors::Cors, server as actix_web_server, App, Body,
    HttpRequest, HttpResponse, Result as AResult
};

use relay::Relay;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

extern crate ws;

use ws::{listen, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender};

struct Server {
    out: Sender,
}

impl Server {
    pub fn send(&mut self, msg: String) -> WsResult<()> {
        self.out.send(msg)
    }
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        self.send("Hello WebSocket".to_owned())
    }

    fn on_message(&mut self, msg: Message) -> WsResult<()> {
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
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // TODO: starta server-tråd för att ta emot HTTP POST
    std::thread::spawn(move || listen("127.0.0.1:8081", |out| Server { out: out }).unwrap());

    let sys = actix_web::actix::System::new("Trivia Application");
    start_server(&config::get_base_url());
    let _ = sys.run();

}

pub fn start_server(base_url: &String) -> () {
    actix_web_server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .handler(
                "/app",
                fs::StaticFiles::new("./app/resources/public/").unwrap(),
            ).resource("/", |r| {
                r.method(http::Method::GET).f(|_| {
                    HttpResponse::Found()
                        .header("LOCATION", "/app/index.html")
                        .finish()
                })
            }).resource("/api/guess", |r| r.method(http::Method::POST).f(guess))
    }).bind(&base_url)
    .unwrap()
    .start();
    info!("Started http server: http://{}", &base_url);
}

pub fn guess(r: &HttpRequest) -> AResult<HttpResponse> {
    Ok(HttpResponse::with_body(StatusCode::OK, Body::Empty))
}
