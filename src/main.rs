#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::{env, io};

mod config;
mod controllers;
mod helpers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let address = format!(
    "{}:{}",
    env::var("APP_HOST").expect("APP_HOTS undefined"),
    env::var("APP_PORT").expect("APP_PORT undefined")
  );

  let server = HttpServer::new(|| {
    App::new()
      .route(
        "/",
        web::get().to(|| HttpResponse::Ok().body("Hello, World")),
      )
      .route("/todos", web::get().to(controllers::get_all_todo))
      .route("/todos/{id}", web::get().to(controllers::get_todo))
      .route("/todos", web::post().to(controllers::add_todo))
      .route("/todos/{id}", web::put().to(controllers::update_todo))
      .route("/todos/{id}", web::delete().to(controllers::destroy_todo))
  })
  .bind(&address)?;

  println!("Server is running http://{}", address);

  server.run().await
}
