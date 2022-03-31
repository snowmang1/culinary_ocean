use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

    log::info!("starting HTTP server at localhost:8080");

    HttpServer::new(|| {
        App::new()
            // allows visiter to view static dir with index file set to index.html
            // this server location is the root path and should only be defined last
            // https://github.com/actix/examples/blob/master/basics/static-files/src/main.rs
            .service(Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
