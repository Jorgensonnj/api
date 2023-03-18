//use sqlx::{ FromRow, types::chrono::{DateTime, Utc} };
//use serde::{Serialize, Deserialize};

//#[derive(Serialize, Deserialize, Debug)]
//pub struct JsonUser {
//    pub username: Option<String>,
//    pub password: Option<String>,
//    pub email: Option<String>,
//    pub discarded: Option<bool>
//}
//
//impl JsonUser {
//    pub fn get_discarded(&self) -> bool {
//        match self.discarded {
//            Some(discarded) => discarded,
//            None => false
//        }
//    }
//}

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

