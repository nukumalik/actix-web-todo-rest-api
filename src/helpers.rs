use crate::models;
use serde::Serialize;
use serde_json;

pub fn result<T: Serialize>(code: u16, data: T, message: &'static str) -> serde_json::Value {
  serde_json::json!(models::Result {
    code,
    data,
    message
  })
}
