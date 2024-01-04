use crate::models::{Account, AccountData};
use crate::errors::ActixError;
use mysql::prelude::Queryable;
use mysql::params;

pub fn create_new_account(
    pool: &mysql::Pool,
    account: AccountData,
    hash_password: &str
) -> Result<Account, ActixError> {
    if account.username.replace(' ', "").trim().is_empty() {
        return Err(ActixError::EmptyAccountName);
    }

    let mut conn = pool.get_conn()?;

    let last_insert_id = insert_account_data(&mut conn, &account.username, hash_password)?;

    if last_insert_id > 0 {
        Ok(Account{
            id: last_insert_id,
            password: String::from(hash_password),
            username: account.username,
        })
    } else {
        Err(ActixError::Unknown)
    }
}

pub fn get_account_data(pool: &mysql::Pool) -> Result<Vec<Account>, ActixError> {
    let mut conn = pool.get_conn()?;

    Ok(select_account_details(&mut conn)?)
}

pub fn update_account_data(pool: &mysql::Pool, id_account: u64, username: &str) -> Result<(), ActixError> {
    let mut conn = pool.get_conn()?;

    Ok(update_account_username(&mut conn, id_account, username)?)
}

pub fn update_account_username(
    conn: &mut mysql::PooledConn,
    account_id: u64,
    new_username: &str,
) -> mysql::error::Result<()> {
    conn.exec_drop(
        "UPDATE accounts SET username = :username WHERE id = :id",
        params! {
            "id" => account_id,
            "username" => new_username,
        },
    )
}

fn select_account_details(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<Account>> {
    conn.query_map(
        "SELECT id, username, password FROM accounts ORDER BY id ASC",
        |(id, username, password)| Account {
            id: id,
            username: username,
            password: password,
        },
    )
}

pub fn insert_account_data(
    conn: &mut mysql::PooledConn,
    username: &str,
    hash_password: &str,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "INSERT INTO accounts (username,password) VALUES (:username,:password)",
        params! {
          "username" => username,
          "password" => hash_password,
        },
    )
    .map(|_| conn.last_insert_id())
}