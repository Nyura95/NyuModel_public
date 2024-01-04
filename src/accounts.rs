use std::sync::{Arc, Mutex};
use crate::{models::Account, errors::ActixError};

#[derive(Clone)]
pub struct AccountDB {
    accounts: Arc<Mutex<Vec<Account>>>,
}

impl AccountDB {
  pub fn new() -> AccountDB {
      AccountDB {
          accounts: Arc::new(Mutex::new(Vec::new())),
      }
  }

  pub fn update_username(&self, account_id: u64, new_username: &str) -> Result<Account, ActixError> {
    let mut accounts = self.accounts.lock().unwrap();

    if let Some(account) = accounts.iter_mut().find(|acc| acc.id == account_id) {
        account.username = new_username.to_string();
        Ok(account.clone())
    } else {
        Err(ActixError::NotFound)
    }
  }

  pub fn add_account(&self, new_account: Account) -> Result<(), ActixError> {
    let mut accounts = self.accounts.lock().unwrap();
    if accounts.iter().any(|account| account.username == new_account.username) {
        return Err(ActixError::SameAccountName);
    }
    accounts.push(new_account);
    Ok(())
  }
  pub fn get_accounts(&self) -> std::sync::MutexGuard<Vec<Account>> {
    self.accounts.lock().unwrap()
  }

  pub fn get_account(&self, id_account: u64) -> Result<Account, ActixError> {
    let accounts = self.accounts.lock().unwrap();

    match accounts.iter().find(|account| account.id == id_account) {
        Some(account) => Ok(account.clone()),
        None => Err(ActixError::NotFound),
    }
  }

  pub fn is_exist(&self, username: &str) -> bool {
    let accounts = self.accounts.lock().unwrap();
    accounts.iter().any(|account| account.username == username)
  }

  pub fn get_account_by_username(&self, username: &str) -> Result<Account, ActixError> {
    let accounts = self.accounts.lock().unwrap();

    match accounts.iter().find(|account| account.username == username) {
        Some(account) => Ok(account.clone()),
        None => Err(ActixError::NotFound),
    }
  }

  pub fn verify_credentials(&self, username: &str, password: &str) -> bool {
    let accounts = self.accounts.lock().unwrap();

    for account in accounts.iter() {
      println!("Vérification du compte: {:?}", account);
      if account.username == username {
        return account.password == self.hash_password(username, password);
      }
    }

    false
  }

  fn hash_md5(&self, password: &str) -> String {
    let digest = md5::compute(password);
    format!("{:x}", digest)
  }

  pub fn hash_password(&self, username: &str, password: &str) -> String {
    let format_password = format!("{}:{}", username, password);
    return self.hash_md5(&format_password);
  }
}