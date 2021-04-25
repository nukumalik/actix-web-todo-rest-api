use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn conn() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL undefined");

  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connection to {}", database_url))
}
