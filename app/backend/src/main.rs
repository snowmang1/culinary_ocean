#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{
    get, middleware, middleware::Logger, post, web, App, Error, HttpResponse, HttpServer,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// query db for email
#[get("/user/{user_email}")]
async fn get_user_email(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    // web::block
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::find_user_by_email(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with that email"));
        Ok(res)
    }
}

// Inserts new user
#[post("/user")]
async fn add_user(
    // db pool for unified connection
    pool: web::Data<DbPool>,
    // json pickup
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::insert_new_user(
            form.user_email.to_owned(),
            form.password.to_owned(),
            form.instructions.to_owned(),
            form.ingredients.to_owned(),
            &conn,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // setup db connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server at localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(add_user)
            .service(get_user_email)
            // allows visiter to view static dir with index file set to index.html
            // this server location is the root path and should only be defined last
            // https://github.com/actix/examples/blob/master/basics/static-files/src/main.rs
            .service(Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
