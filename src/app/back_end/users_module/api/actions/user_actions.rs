use sqlx::{Pool, Execute, QueryBuilder, Postgres, Error, postgres::PgQueryResult};
use super::super::super::super::shared_module::api::models::shared_models::*;
use std::collections::HashMap;

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

pub async fn read_one(pool: &Pool<Postgres>, user_id: &i32) -> Result<Vec<User>, Error> {
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

pub async fn read_all(pool: &Pool<Postgres>, query_map: &HashMap<String, String>) -> Result<Vec<User>, Error> {

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT * FROM users"
    );

    let username_key  = String::from("username");
    let email_key     = String::from("email");
    let discarded_key = String::from("discarded");

    if query_map.contains_key(&username_key) ||
       query_map.contains_key(&email_key)    ||
       query_map.contains_key(&discarded_key)
    {
        query_builder.push(" WHERE ");
        let mut seperator = query_builder.separated(" AND ");

        if let Some(value) = query_map.get(&username_key){
            seperator.push(format!("{} LIKE '%{}%'", username_key, value));
        };

        if let Some(value) = query_map.get(&"email".to_string()){
            seperator.push(format!("{} LIKE '%{}%'", email_key, value));
        };

        if let Some(value) = query_map.get(&"discarded".to_string()){
            seperator.push(format!("{} = '{}'", discarded_key, value));
        };
    }

    //println!("{}",query_builder.build().sql());

    sqlx::query_as::<_, User>(query_builder.build().sql())
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
