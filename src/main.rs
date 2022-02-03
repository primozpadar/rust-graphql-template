#[macro_use]
extern crate diesel;

use std::env;
use std::io;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod graphql;
mod handlers;
mod schema;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").unwrap();
    let pool = db::connect(&db_url);

    println!("Server Running on port 8080 - http://localhost:8080/graphql");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql::register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
