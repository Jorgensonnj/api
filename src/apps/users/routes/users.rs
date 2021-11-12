use actix_web::{web::{Data, Path}, HttpRequest, HttpResponse, Responder};
use crate::apps::users::models::users::*;
use sqlx::{PgPool};
use tracing::instrument;
//use serde_json::Value;

// /users
#[instrument]
pub async fn create_user(_req: HttpRequest, pool: Data<PgPool>) -> impl Responder {
    let result = String::from("create_user");

    HttpResponse::Ok().json(result)
}

// /users
#[instrument]
pub async fn read_users(_req: HttpRequest, pool: Data<PgPool>) -> impl Responder {
    //let result = String::from("read_users");


    //TODO use query_as! macro instead

    // query
    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })
    .unwrap();

    HttpResponse::Ok().json(result)
}

// /users/{user_id}
#[instrument]
pub async fn read_user(_req: HttpRequest, user_id: Path<i32>, pool: Data<PgPool>) -> impl Responder {
    let result = format!("{} - {}","read_user", user_id);

    //// query
    //let result = sqlx::query_as!(
    //    User,
    //    "SELECT * FROM users
    //     WHERE user_id = ?",
    //    user_id.into_inner(),
    //)
    //.fetch_all(pool.get_ref())
    //.await
    //.map_err(|e| {
    //    tracing::error!("Failed to execute query: {:?}", e);
    //    HttpResponse::InternalServerError().finish()
    //})
    //.unwrap();

    HttpResponse::Ok().json(result)
}

// /users/{user_id}
#[instrument]
pub async fn update_user(_req: HttpRequest, user_id: Path<i32>, pool: Data<PgPool>) -> impl Responder {
    let response = format!("{} - {}","update_user", user_id);

    HttpResponse::Ok().json(response)
}

// /users/{user_id}
#[instrument]
pub async fn delete_user(_req: HttpRequest, user_id: Path<i32>, pool: Data<PgPool>) -> impl Responder {
    let response = format!("{} - {}","delete_user", user_id);

    HttpResponse::Ok().json(response)
}
