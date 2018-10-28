use actix_web::HttpResponse;
use actix_web::{fs, http, middleware, middleware::cors::Cors, server as actix_server, App};
use res;

pub fn start_server(base_url: &String) -> () {
    actix_server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(cors_web_config)
            .handler(
                "/app",
                fs::StaticFiles::new("./app/resources/public/").unwrap(),
            ).resource("/", |r| {
                r.method(http::Method::GET).f(|_| {
                    HttpResponse::Found()
                        .header("LOCATION", "/app/index.html")
                        .finish()
                })
            })
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
        }).resource("/ws/", |r| {
            r.method(http::Method::GET).f(res::websocket::ws_index)
        }).register()
}
