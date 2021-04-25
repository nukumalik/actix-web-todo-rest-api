use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::schema::todos;
use crate::schema::todos::dsl::todos as all_todos;

#[derive(Serialize)]
pub struct Result<T> {
  pub code: u16,
  pub data: T,
  pub message: &'static str,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub description: String,
  pub is_complete: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
  pub title: String,
  pub description: String,
  pub is_complete: bool,
}

impl Todo {
  #[allow(dead_code)]
  pub fn get(id: i32, conn: &SqliteConnection) -> Vec<Todo> {
    all_todos
      .find(id)
      .load::<Todo>(conn)
      .expect(&format!("Failed to get todo with ID {}", id))
  }

  #[allow(dead_code)]
  pub fn get_all(conn: &SqliteConnection) -> Vec<Todo> {
    all_todos
      .order(todos::id.desc())
      .load::<Todo>(conn)
      .expect("Failed to get all todo")
  }

  pub fn add(todo: NewTodo, conn: &SqliteConnection) -> bool {
    diesel::insert_into(todos::table)
      .values(&todo)
      .execute(conn)
      .is_ok()
  }

  #[allow(dead_code)]
  pub fn update(id: i32, conn: &SqliteConnection, todo: NewTodo) -> bool {
    use crate::schema::todos::dsl::{description as d, is_complete as c, title as t};

    let NewTodo {
      title,
      description,
      is_complete,
    } = todo;

    diesel::update(all_todos.find(id))
      .set((d.eq(description), c.eq(is_complete), t.eq(title)))
      .execute(conn)
      .is_ok()
  }

  #[allow(dead_code)]
  pub fn destroy(id: i32, conn: &SqliteConnection) -> bool {
    if Todo::get(id, conn).is_empty() {
      return false;
    };

    diesel::delete(all_todos.find(id)).execute(conn).is_ok()
  }

  #[allow(dead_code)]
  pub fn get_completed(conn: &SqliteConnection) -> Vec<Todo> {
    all_todos
      .filter(todos::is_complete.eq(true))
      .load::<Todo>(conn)
      .expect("Failed to get completed todos")
  }
}
