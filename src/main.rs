#[allow(unused)]
#[macro_use] 
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[allow(unused)]
#[macro_use]
extern crate askama;
extern crate reqwest;

extern crate actix_web;
use actix_web::actix;

mod res;
mod webserver;
mod config;

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = actix::System::new("Trivia Application");    
    webserver::start_server(&config::get_base_url());
    let _ = sys.run();
}
