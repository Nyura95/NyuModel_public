use actix_web::{get, web, Responder};
use crate::models::BasicResponse;

#[get("/")]
pub async fn health_checker_handler() -> impl Responder {
  let obj = BasicResponse {
    data: String::from("Build Simple CRUD API with Rust"),
  };

  web::Json(obj)
}

