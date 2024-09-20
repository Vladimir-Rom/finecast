mod config;
mod handlers;
mod rss;

use ::config::{Config, File};
use actix_web::{HttpServer, App, middleware, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
    .add_source(File::with_name("config.yaml"))
    .build()
    .unwrap();
    let conf_root = settings.try_deserialize::<crate::config::Root>().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(handlers::hello)
            .service(handlers::conf)
            .service(handlers::rss_handler)
            .app_data(web::Data::new(conf_root.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
