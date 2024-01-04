use crate::models::BasicResponseError;
use actix_web::{http::StatusCode, HttpResponse, error::BlockingError};
use derive_more::{Display, Error, From};
use serde_json::to_string;

#[derive(Debug, Display, Error, From)]
pub enum ActixError {
  EmptyAccountName,
  SameAccountName,
  NotFound,
  UnAuthorized,
  MysqlError(mysql::Error),
  BlockingError(BlockingError),
  Unknown,
}

impl actix_web::error::ResponseError  for ActixError {
  fn status_code(&self) -> StatusCode {
      match self {
        ActixError::EmptyAccountName | ActixError::NotFound => StatusCode::BAD_REQUEST,
        ActixError::SameAccountName => StatusCode::CONFLICT,
        ActixError::UnAuthorized => StatusCode::UNAUTHORIZED,
        ActixError::MysqlError(_) | ActixError::Unknown | ActixError::BlockingError(_) => {
          StatusCode::INTERNAL_SERVER_ERROR
        }
      }
  }
  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    let status_code = self.status_code();
    let error_message = match self {
      ActixError::EmptyAccountName => "Account name cannot be empty.",
      ActixError::SameAccountName => "Account already exist.",
      ActixError::UnAuthorized => "Not authorize.",
      ActixError::NotFound => "Not found.",
      ActixError::MysqlError(_) | ActixError::BlockingError(_) | ActixError::Unknown => {
        "Database error occurred."
      }
    };

    let error_response = BasicResponseError {
      error: error_message.to_string(),
      code: status_code.as_u16(),
    };
    let json_body = to_string(&error_response)
      .unwrap_or_else(|_| "{\"error\": \"Error serializing response\"}".to_string());

    HttpResponse::build(status_code)
      .content_type("application/json")
      .body(json_body)
  }
}
