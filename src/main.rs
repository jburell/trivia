include!("externs.rs");
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
