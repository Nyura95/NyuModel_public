use crate::models::{Claims, AccessToken};
use crate::errors::ActixError;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use mysql::prelude::Queryable;
use mysql::params;
use std::env;

pub fn create_new_access_token(
    pool: &mysql::Pool,
    id_account: u64,
) -> Result<AccessToken, ActixError> {

    let expiration = Utc::now().checked_add_signed(Duration::minutes(10))
        .ok_or(ActixError::Unknown)?
        .timestamp();

    let claims = Claims{
        admin: false,
        exp: expiration,
        id_account: id_account,
    };

    let token_secret = match env::var("TOKEN_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err(ActixError::Unknown),
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(token_secret.as_ref())).map_err(|_| ActixError::Unknown)?;

    let refresh_token_secret = match env::var("REFRESH_TOKEN_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err(ActixError::Unknown),
    };
    let refresh_token = encode(&Header::default(), &claims, &EncodingKey::from_secret(refresh_token_secret.as_ref())).map_err(|_| ActixError::Unknown)?;

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_access_token(&mut conn, claims.id_account, &token, &refresh_token).unwrap();
    if last_insert_id > 0 {
        Ok(AccessToken{
            id: last_insert_id,
            access_token: token,
            id_account: claims.id_account,
            refresh_token: refresh_token,
        })
    } else {
        Err(ActixError::Unknown)
    }
}

pub fn insert_access_token(
    conn: &mut mysql::PooledConn,
    id_account: u64,
    access_token: &str,
    refresh_token: &str,
) -> mysql::error::Result<u64> {
    conn.exec_drop("INSERT INTO access_token (id_account,access_token,refresh_token) VALUES (:id_account,:access_token,:refresh_token)", params! {
      "id_account" => id_account,
      "access_token" => access_token,
      "refresh_token" => refresh_token,
    },
  ).map(|_| conn.last_insert_id())
}
