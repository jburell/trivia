use actix::prelude::*;
use actix_web::HttpResponse;
use actix_web::{ws::WsWriter, ws, fs, http, middleware, middleware::cors::Cors, server as actix_server, App};
use res;
use relay::Relay;
use ::res::websocket::MyWebSocket;
use std::sync::{ Mutex, Arc };

pub struct WsServer {
    relay: Arc<Mutex<Relay>>,
}
/*
static application: App = App::new();
            

impl WsServer {
    pub fn new(relay: Arc<Mutex<Relay>>) -> WsServer {
        WsServer {relay: relay }
    }

    pub fn start_server(&self, base_url: &String) -> () {
        // Couple RelayActor and WsActor
        //let relay_clone = relay.clone();
        application.middleware(middleware::Logger::default())
            .configure(|a| self.cors_web_config(a))
            .handler(
                "/app",
                fs::StaticFiles::new("./app/resources/public/").unwrap(),
            ).resource("/", |r| {
                r.method(http::Method::GET).f(|_| {
                    HttpResponse::Found()
                        .header("LOCATION", "/app/index.html")
                        .finish()
                })
            }); 

        actix_server::new(|| application).bind(&base_url)
            .unwrap()
            .start();
        info!("Started http server: http://{}", &base_url);
    }

    fn cors_web_config(&self, app: App) -> App {
        let relay_clone = self.relay.clone();
        Cors::for_app(app)
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .max_age(3600)
            .resource("/index", |r| {
                r.method(http::Method::GET).with(res::trivia::index)
            }).resource("/ws/", |r| {
                r.method(http::Method::GET).f(|r| {
                        let socket_actor = MyWebSocket::new(relay_clone);   
                        //socket_actor.send_text("Hello");
                        //let addr = actix::Actor::create(|ctx| socket_actor);
                        //println!("Socket actor addr: {:?}", addr);
                        //relay.set_reciever_addr(addr);
                        ws::start(r, socket_actor)
                })
            }).register()
    }
}*/
