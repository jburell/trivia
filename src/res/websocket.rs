use actix::prelude::*;
use actix_web::{ws::WsWriter, ws, Error, HttpRequest, HttpResponse};
use std::time::{Duration, Instant};
use relay::Relay;
use std::cell::RefCell;
use std::sync::{ Mutex, Arc };


/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
/*pub fn ws_index(r: &HttpRequest) -> Result<HttpResponse, Error> {
    let socket_actor = MyWebSocket::new();
    //let addr = socket_actor.create();
    //println!("Socket actor addr: {:?}", addr);
    ws::start(r, socket_actor)
}*/

/// websocket connection is long running connection, it easier
/// to handle with an actor
//#[derive(Debug)]
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    relay: Arc<Mutex<Relay>>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        ctx.send_text("text");
        let addr = ctx.address();
        let mut relay_lock = (*self.relay).lock().unwrap();
        relay_lock.set_reciever_addr(addr);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        info!("WS Message recieved: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
                ctx.text("Hello!".to_owned());
            }
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}

impl MyWebSocket {
    pub fn new(relay: Arc<Mutex<Relay>>) -> Self {
        Self { hb: Instant::now(), relay: relay, }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("ping");
        });
    }
}
