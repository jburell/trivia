include!("externs.rs");
use actix_web::actix as actix_server;

mod config;
mod res;
mod webserver;
mod relay;

use relay::Relay;

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //let relay = Relay::new();

    let sys = actix_server::System::new("Trivia Application");
    webserver::start_server(&config::get_base_url(), relay);
    let _ = sys.run();
}
