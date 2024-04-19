
use reqwest::{Client, Error};
use std::collections::HashMap;

pub async fn login_action(address: &String) -> Result<HashMap<String, String>, Error> {

    Client::new()
        .post(format!("{}/api/v0/logout", address))
        .send()
        .await
        .map_err(|error| {
            tracing::error!("failed to make request: {:?}", error);
            error
        })?
        .json::<HashMap<String, String>>()
        .await
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

