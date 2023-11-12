use sqlx::{Pool, Postgres, Error, postgres::PgQueryResult};
use super::super::super::super::shared_module::api::models::shared_models::*;

pub async fn create(pool: &Pool<Postgres>, json_user: &JsonUser) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (username, password, email, discarded)
            VALUES ($1, $2, $3, $4)
            RETURNING *",
        json_user.username,
        json_user.password,
        json_user.email,
        json_user.get_discarded()
   )
   .fetch_one(pool)
   .await
   .map_err(|error| {
       tracing::error!("failed to execute create query: {:?}", error);
       error
   })
}

pub async fn read_one(pool: &Pool<Postgres>, user_id: i32) -> Result<Vec<User>, Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        tracing::error!("failed to execute read_one query: {:?}", error);
        error
    })
}

pub async fn read_all(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        tracing::error!("failed to execute read_all query: {:?}", error);
        error
    })
}

pub async fn update(
    pool: &Pool<Postgres>,
    user_id: i32,
    json_user: JsonUser
) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "UPDATE users SET
            username  = $1,
            password  = $2,
            email     = $3,
            discarded = $4
            WHERE user_id = $5
            RETURNING *",
        json_user.username,
        json_user.password,
        json_user.email,
        json_user.discarded,
        user_id,
   )
   .fetch_one(pool)
   .await
   .map_err(|error| {
       tracing::error!("failed to execute update query: {:?}", error);
       error
   })
}

pub async fn delete(pool: &Pool<Postgres>, user_id: i32) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        "DELETE FROM users WHERE user_id = $1",
        user_id
    )
    .execute(pool)
    .await
    .map_err(|error| {
        tracing::error!("failed to execute delete query: {:?}", error);
        error
    })
}

//pub async fn search_one(pool: &Pool<Postgres>, json_user: &JsonUser) -> Result<User, Error> {
//    sqlx::query_as::<_, User>(
//        "SELECT * FROM users
//            WHERE username = $1
//            AND email = $2"
//        )
//        .bind(&json_user.username)
//        .bind(&json_user.email)
//        .fetch_one(pool)
//        .await
//        .map_err(|error| {
//            tracing::error!("failed to execute search_one query: {:?}", error);
//            error
//        })
//}
