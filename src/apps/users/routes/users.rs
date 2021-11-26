use actix_web::{web::{Data, Path}, HttpRequest, HttpResponse, Responder};
use crate::apps::users::models::users::*;
use sqlx::{Any, Pool, Error};
use tracing::instrument;
//use serde_json::Value;

// /users
#[instrument]
pub async fn create_user(_req: HttpRequest, result_pool: Data<Result<Pool<Any>, Error>>) -> impl Responder {
    let result = String::from("create_user");

    HttpResponse::Ok().json(result)
}

// /users
#[instrument]
pub async fn read_users(_req: HttpRequest, data_pool: Data<Result<Pool<Any>, Error>>) -> impl Responder {
    //let result = String::from("read_users");

    // Unwrap 
    let data_pool = data_pool.get_ref();

    // is there a db connection
    match data_pool {
        Ok(pool) => {
            // get all users query
            let result = sqlx::query_as::<_, User>( "SELECT * FROM users" )
                .fetch_all(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to execute query: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                })
                .unwrap();
            HttpResponse::Ok().json(result)
        },
        Err(e) => { 
            // no db was found
            tracing::error!("no database found: {:?}", e);
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

// /users/{user_id}
#[instrument]
pub async fn read_user(_req: HttpRequest, user_id: Path<i32>, result_pool: Data<Result<Pool<Any>, Error>>) -> impl Responder {
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
pub async fn update_user(_req: HttpRequest, user_id: Path<i32>, result_pool: Data<Result<Pool<Any>, Error>>) -> impl Responder {
    let response = format!("{} - {}","update_user", user_id);

    HttpResponse::Ok().json(response)
}

// /users/{user_id}
#[instrument]
pub async fn delete_user(_req: HttpRequest, user_id: Path<i32>, result_pool: Data<Result<Pool<Any>, Error>>) -> impl Responder {
    let response = format!("{} - {}","delete_user", user_id);

    HttpResponse::Ok().json(response)
}
