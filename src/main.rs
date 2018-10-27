#[allow(unused)]
#[macro_use] 
extern crate log;
extern crate env_logger;

extern crate actix_web;
use actix_web::{
    http, middleware, server, App, Error, HttpResponse, Query,
};

use std::collections::HashMap;

#[allow(unused)]
#[macro_use]
extern crate askama;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse, Error>  {
    let s = if let Some(name) = query.get("name") {
        IndexTemplate {
            name: name,
        }.render().unwrap()
    } else {
        IndexTemplate {
            name: "world",
        }.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

const HOSTNAME: &'static str = "127.0.0.1";
const PORT: &'static str = "8080";

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let sys = actix::System::new("Trivia WebApp");
    let base_url = format!("{}:{}", HOSTNAME, PORT);

    server::new(|| {
        App::new().middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
    }).bind(&base_url)
        .unwrap()
        .start();

    info!("Started http server: {}", &base_url);
    let _ = sys.run();
}
