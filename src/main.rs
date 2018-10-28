#[allow(unused)]
#[macro_use] 
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate actix_web;
use actix_web::{
    actix,
    http, middleware, server, App, HttpResponse, Query, Json, Result,
    http::{header, Method},
    middleware::cors::Cors,
};

use std::collections::HashMap;

#[allow(unused)]
#[macro_use]
extern crate askama;
extern crate reqwest;

//use actix_web::{actix, http, middleware, server, App, Error, HttpResponse, Query};
use askama::Template;
use reqwest::Client;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(Deserialize, Debug)]
struct TriviaResponse {
    response_code: u32,
    results: Vec<TriviaSpec>,
}

#[derive(Deserialize, Debug)]
struct TriviaSpec {
    category: String,
    #[serde(rename = "type")]
    ty: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

pub fn get_from_ws(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    println!("=========={:?}=========", query);
    Ok(HttpResponse::Ok().content_type("text/html").body("{\"value\": [1 2 3]}"))
}

pub fn options(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    println!("=========={:?}=========", query);
    Ok(HttpResponse::Ok().content_type("text/html").body("(msg \"hello\")."))
}

const HOSTNAME: &'static str = "127.0.0.1";
const PORT: &'static str = "8080";

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = actix::System::new("tera-example");
    let base_url = format!("{}:{}", HOSTNAME, PORT);
    start_server(&base_url);
    let _ = sys.run();
}

fn start_server(base_url: &String) -> () {
    server::new(|| {
        App::new().middleware(middleware::Logger::default())
             .configure(|app| {
                Cors::for_app(app)
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .max_age(3600)
                    .resource("/chsk", |r| {
                        r.method(Method::GET).with(get_from_ws);
                        r.method(Method::OPTIONS).with(options);
                    })
                    .resource("/", |r| r.method(http::Method::GET).with(index))
                    .register()
            })
            
    }).bind(&base_url)
    .unwrap()
    .start();
    info!("Started http server: http://{}", &base_url);
}

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let fallback_name = &"world".to_string();
    let name = query.get("name").unwrap_or(fallback_name);

    let trivia_response: TriviaResponse = reqwest::get("https://opentdb.com/api.php?amount=1")
        .unwrap()
        .json()
        .unwrap();
    println!("{:?}", trivia_response);

    let s = IndexTemplate { name: name }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
