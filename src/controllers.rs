use actix_web::{web, HttpRequest, Responder};
use serde::Serialize;

use crate::config;
use crate::helpers;
use crate::models;

#[derive(Serialize)]
struct ResultProcess {
  code: i16,
  message: &'static str,
}

pub async fn get_all_todo() -> impl Responder {
  let products = models::Todo::get_all(&config::conn());
  let data = if products.len() > 0 {
    products
  } else {
    Vec::new()
  };
  web::Json(helpers::result(200, data, "Success to get all todo"))
}

pub async fn get_todo(req: HttpRequest) -> impl Responder {
  let id = req.match_info().get("id").unwrap(); // get path id
  let product = models::Todo::get(id.parse::<i32>().unwrap(), &config::conn()); // Parse id from &str to i32
  if product.len() < 1 {
    web::Json(helpers::result(404, vec![0; 0], "Todo not found"))
  } else {
    web::Json(helpers::result(200, product, "Success to get todo"))
  }
}

pub async fn add_todo(form: web::Form<models::NewTodo>) -> impl Responder {
  let todo = models::NewTodo {
    title: form.title.clone(),
    description: form.title.clone(),
    is_complete: form.is_complete,
  };

  if models::Todo::add(todo, &config::conn()) {
    web::Json(ResultProcess {
      code: 200,
      message: "Success to add todo",
    })
  } else {
    web::Json(ResultProcess {
      code: 401,
      message: "Failed to add todo",
    })
  }
}

pub async fn update_todo(req: HttpRequest, form: web::Form<models::NewTodo>) -> impl Responder {
  let id = req.match_info().get("id").unwrap();
  let data = models::NewTodo {
    title: form.title.clone(),
    description: form.description.clone(),
    is_complete: form.is_complete,
  };

  let updated = models::Todo::update(id.parse::<i32>().unwrap(), &config::conn(), data);
  match updated {
    true => web::Json(ResultProcess {
      code: 200,
      message: "Success to update todo",
    }),
    false => web::Json(ResultProcess {
      code: 400,
      message: "Failed to update todo",
    }),
  }
}

pub async fn destroy_todo(req: HttpRequest) -> impl Responder {
  let id = req.match_info().get("id").unwrap();
  let destroyed = models::Todo::destroy(id.parse::<i32>().unwrap(), &config::conn());
  match destroyed {
    true => web::Json(ResultProcess {
      code: 200,
      message: "Success to delete todo",
    }),
    false => web::Json(ResultProcess {
      code: 400,
      message: "Failed to delete todo",
    }),
  }
}
