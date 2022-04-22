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
    email: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let email = email.into_inner();

    // web::block
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::find_user_by_email(email, &conn)
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
        actions::insert_new_user(form.user_email.to_owned(), form.password.to_owned(), 
                                 form.instructions.to_owned(),
                                 form.ingredients.to_owned(),
                                 &conn)
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    // do to concurency issues only one test can run on
    // any given cargo test instance

    #[actix_web::test]
    async fn email_query_test() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        dotenv::dotenv().ok();

        let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<SqliteConnection>::new(connspec);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(get_user_email)
                .service(add_user),
        )
        .await;

        // Insert a user
        let req = test::TestRequest::post()
            .uri("/user")
            .set_json(&models::NewUser {
                user_email: String::from("test@mail.com"),
                password: String::from("test"),
                instructions: String::from("please test"),
                ingredients: String::from("test pear")
            })
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.user_email, "test@mail.com");

        // Get a user
        let req = test::TestRequest::get()
            .uri(&format!("/user/{}", resp.user_email))
            .to_request();

        let resp: models::User = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.user_email, "test@mail.com");

        // Delete new user from table
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(user_email.eq(resp.user_email)))
            .execute(&pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test user from table");
    }

}
