use actix_web::{post, web, Responder, Result};
use crate::accounts::AccountDB;
use crate::errors::ActixError;
use crate::persistance::access::create_new_access_token;
use crate::models::LoginData;


#[post("/login")]
async fn login_handler(login_data: web::Json<LoginData>, db: web::Data<mysql::Pool>, accounts: web::Data<AccountDB>) -> Result<impl Responder, ActixError> {
  if !accounts.is_exist(&login_data.username) {
    return Err(ActixError::SameAccountName);
  }

  if !accounts.verify_credentials(&login_data.username, &login_data.password) {
    return Err(ActixError::UnAuthorized);
  }

  let access_token = web::block(move || create_new_access_token(&db, accounts.get_account_by_username(&login_data.username)?.id)).await??;

  Ok(web::Json(access_token))
}
