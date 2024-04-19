use sqlx::{ FromRow, types::chrono::{ DateTime, Utc } };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_on: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub discarded: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub discarded: Option<bool>
}

impl JsonUser {
    pub fn get_discarded(&self) -> bool {
        self.discarded.unwrap_or(false)
    }
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

