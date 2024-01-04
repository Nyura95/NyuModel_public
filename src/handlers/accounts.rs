use actix_web::{get, post, put, web, Responder, Result};
use crate::accounts::AccountDB;
use crate::persistance::accounts::{create_new_account, update_account_data};
use crate::errors::ActixError;
use crate::models::{Account, AccountData, QueryPageOptions, UpdateAccountData};


#[get("/accounts")]
async fn accounts_list_handler(opts: web::Query<QueryPageOptions>, accounts: web::Data<AccountDB>) -> impl Responder {
  let accounts_lock  = accounts.get_accounts();

  let limit = opts.limit.unwrap_or(10);
  let offset = (opts.page.unwrap_or(1) - 1) * limit;

  let results: Vec<Account> = accounts_lock.iter().skip(offset).take(limit) .cloned().collect();

  web::Json(results)
}

#[post("/accounts")]
async fn accounts_create_handler(account_data: web::Json<AccountData>, db: web::Data<mysql::Pool>, accounts: web::Data<AccountDB>) -> Result<impl Responder, ActixError>{
  if accounts.is_exist(&account_data.username) {
    return Err(ActixError::SameAccountName);
  }

  let hash_password = accounts.hash_password(&account_data.username, &account_data.password);

  let account = web::block(move || create_new_account(&db, account_data.0, &hash_password)).await??;

  accounts.add_account(account.clone())?;

  Ok(web::Json(account))
}

#[get("/accounts/{id_account}")]
async fn account_get_handler(path: web::Path<u64>, accounts: web::Data<AccountDB>) -> Result<impl Responder, ActixError>{
  let id_account = path.into_inner();

  let account = accounts.get_account(id_account)?;

  Ok(web::Json(account))
}

#[put("/accounts/{id_account}")]
async fn account_update_handler(path: web::Path<u64>, update: web::Json<UpdateAccountData>, db: web::Data<mysql::Pool>, accounts: web::Data<AccountDB>) -> Result<impl Responder, ActixError>{
  let id_account = path.into_inner();

  let account = accounts.update_username(id_account, &update.username)?;
  web::block(move || update_account_data(&db, id_account, &update.username)).await??;
  
  Ok(web::Json(account))
}

