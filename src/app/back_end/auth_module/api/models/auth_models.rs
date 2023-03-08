use sqlx::{ FromRow, types::chrono::{DateTime, Utc} };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct AuthUser {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_on: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub discarded: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthJsonUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

