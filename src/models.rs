use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: u64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccessToken {
    pub id: u64,
    pub id_account: u64,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct QueryPageOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicResponse {
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountData {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdateAccountData {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicResponseError {
    pub error: String,
    pub code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id_account: u64,
    pub admin: bool,
    pub exp: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationMiddleware {
    pub user_id: String,
    pub admin: bool,
}


#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
