//use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String,
}

//impl Role {
//    pub fn from_str(role: &str) -> Role {
//        match role {
//            "Admin" => Role::Admin,
//            _ => Role::User,
//        }
//    }
//}
//
//impl Display for Role {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//        match self {
//            Role::User => write!(f, "User"),
//            Role::Admin => write!(f, "Admin"),
//        }
//    }
//}

