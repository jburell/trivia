use actix::prelude::*;
use actix_web::{
    fs, http, http::Method, middleware, middleware::cors::Cors, server as actix_server, ws, App,
};
use actix_web::{Error, HttpRequest, HttpResponse};
use res;
use serde_json::Error as SerdeError;
use std::time::{Duration, Instant};

pub fn start_server(base_url: &String) -> () {
    actix_server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(cors_web_config)
            .handler(
                "/app",
                fs::StaticFiles::new("./app/resources/public/").unwrap(),
            )
    }).bind(&base_url)
    .unwrap()
    .start();
    info!("Started http server: http://{}", &base_url);
}

fn cors_web_config(app: App) -> App {
    Cors::for_app(app)
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .max_age(3600)
        .resource("/index", |r| {
            r.method(http::Method::GET).with(res::trivia::index)
        }).resource("/ws/", |r| r.method(http::Method::GET).f(ws_index))
        .register()
}

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
fn ws_index(r: &HttpRequest) -> Result<HttpResponse, Error> {
    ws::start(r, MyWebSocket::new())
}

// TODO: consider doing some clever enum stuff here...
#[derive(Deserialize, Debug)]
struct TriviaDing {
    team_token: String,
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("ws: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => self.hb = Instant::now(),
            ws::Message::Text(text) => {
                let r: Result<TriviaDing, SerdeError> = serde_json::from_str(&text);
                match r {
                    Ok(_ding) => ctx.text("u dinged"),
                    Err(e) => {
                        println!("  {:?}", e);
                        ctx.text("u failed")
                    }
                };
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            println!("ws pinging");
            ctx.ping("ping");
        });
    }
}
