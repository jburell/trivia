use ::res;
use actix_web::{
    http, middleware, server as actix_server, App,
    http::Method,
    fs,
    middleware::cors::Cors,
};

pub fn start_server(base_url: &String) -> () {
    actix_server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(cors_config)
            .handler(
                "/app",
                fs::StaticFiles::new("./app/resources/public/").unwrap())
    }).bind(&base_url)
    .unwrap()
    .start();
    info!("Started http server: http://{}", &base_url);
}

fn cors_config(app: App) -> App {
    Cors::for_app(app)
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .max_age(3600)
        .resource("/chsk", |r| {
            r.method(Method::GET).with(res::websocket::get_from_ws);
            r.method(Method::OPTIONS).with(res::websocket::options);
        })
        .resource("/index", |r| r.method(http::Method::GET).with(res::trivia::index))
        .register()
}