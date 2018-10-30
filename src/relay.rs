use actix::prelude::*;
use actix_web::ws;
use std::time::{Instant, Duration};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Relay {
    hb: Instant,
}

impl Relay {
    fn new() -> Self {
        Self { hb: Instant::now() }
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

    pub fn send_message(&mut self) {
        //<Self as Actor<Context=ws::WebsocketContext<Relay>>::text<>(self, "Custom message".to_owned());
        <Self as ws::WsWriter>::send_text(self, "Custom message".to_owned());
    }
}

impl ws::WsWriter for Relay {
    
}

impl Actor for Relay {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for Relay {
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