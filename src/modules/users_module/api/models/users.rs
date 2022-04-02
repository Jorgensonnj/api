use sqlx::{Pool, Postgres, Error, FromRow, types::chrono::{DateTime, Utc}, postgres::PgQueryResult};
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
        match self.discarded {
            Some(discarded) => discarded,
            None => false
        }
    }
}

pub async fn create(pool: &Pool<Postgres>, json_user: JsonUser) -> Result<PgQueryResult, Error> {
    sqlx::query(
        "INSERT INTO users (username, password, email, discarded)
            VALUES ($1, $2, $3, $4)"
        )
        .bind(&json_user.username)
        .bind(&json_user.password)
        .bind(&json_user.email)
        .bind(&json_user.get_discarded())
        .execute(pool)
        .await
        .map_err(|error| {
            tracing::error!("failed to execute create_user query: {:?}", error);
            error
        })
}

pub async fn read_one(pool: &Pool<Postgres>, user_id: i32) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|error| {
            tracing::error!("failed to execute read_user query: {:?}", error);
            error
        })
}

pub async fn read_all(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    sqlx::query_as::<_, User>(
        "SELECT * FROM users"
        )
        .fetch_all(pool)
        .await
        .map_err(|error| {
            tracing::error!("failed to execute read_users query: {:?}", error);
            error
        })
}

pub async fn update(
    pool: &Pool<Postgres>,
    user_id: i32,
    json_user: JsonUser
) -> Result<PgQueryResult, Error> {
    sqlx::query(
        "UPDATE users SET
            username  = $1,
            password  = $2,
            email     = $3,
            discarded = $4
            WHERE user_id = $5"
        )
        .bind(&json_user.username)
        .bind(&json_user.password)
        .bind(&json_user.email)
        .bind(&json_user.discarded)
        .bind(&user_id)
        .execute(pool)
        .await
        .map_err(|error| {
            tracing::error!("failed to execute update_user query: {:?}", error);
            error
        })
}

pub async fn delete(pool: &Pool<Postgres>, user_id: i32) -> Result<PgQueryResult, Error> {
    sqlx::query(
        "DELETE FROM users WHERE user_id = $1"
        )
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|error| {
            tracing::error!("failed to execute delete_user query: {:?}", error);
            error
        })
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

