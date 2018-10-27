extern crate actix_web;
#[allow(unused)]
#[macro_use]
extern crate askama;
extern crate env_logger;
#[allow(unused)]
#[macro_use]
extern crate log;

use actix_web::{actix, http, middleware, server, App, Error, HttpResponse, Query};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

const HOSTNAME: &'static str = "127.0.0.1";
const PORT: &'static str = "8080";

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();
<<<<<<< d02c256c91c1e290813edf2a2e22fb249c8f280c
    let sys = actix::System::new("Trivia WebApp");
=======

    let sys = actix::System::new("tera-example");
>>>>>>> Re-group things
    let base_url = format!("{}:{}", HOSTNAME, PORT);
    start_server(&base_url);
    let _ = sys.run();
}

fn start_server(base_url: &String) -> () {
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
    }).bind(&base_url)
    .unwrap()
    .start();
    info!("Started http server: http://{}", &base_url);
<<<<<<< d02c256c91c1e290813edf2a2e22fb249c8f280c
    let _ = sys.run();
=======
}

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let s = IndexTemplate {
        name: query.get("name").unwrap_or(&"world".to_string()),
    }.render()
    .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
>>>>>>> Re-group things
}
