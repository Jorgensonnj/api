use actix_web::{web::{Json, Data, Path}, HttpRequest, HttpResponse, Responder};
use crate::apps::users::models::users::*;
use sqlx::{Pool, Postgres, Error};
use tracing::instrument;

// /users
#[instrument]
pub async fn create_user(_req: HttpRequest, data_pool: Data<Result<Pool<Postgres>, Error>>, payload: Json<JsonUser>) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get all users query
            let result = sqlx::query(
                "
                INSERT INTO users (username, password, email, discarded)
                    VALUES ($1, $2, $3, $4)
                "
                )
                .bind(&payload.username)
                .bind(&payload.password)
                .bind(&payload.email)
                .bind(&payload.get_discarded())
                .execute(pool)
                .await
                .map_err(|error| {
                    tracing::error!("failed to execute create_user query: {:?}", error);
                    error
                });

            match result {
                Ok(_) => HttpResponse::Created().finish(),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => { 
            // no db was found
            tracing::error!("no database found: {:?}", error);
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

// /users
#[instrument]
pub async fn read_users(_req: HttpRequest, data_pool: Data<Result<Pool<Postgres>, Error>>) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get all users query
            let result = sqlx::query_as::<_, User>(
                    "
                    SELECT * FROM users
                    "
                )
                .fetch_all(pool)
                .await
                .map_err(|error| {
                    tracing::error!("failed to execute read_users query: {:?}", error);
                    error
                });

            match result {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
        },
        Err(error) => { 
            // no db was found
            tracing::error!("no database found: {:?}", error);
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

// /users/{user_id}
#[instrument]
pub async fn read_user(_req: HttpRequest, user_id: Path<i32>, data_pool: Data<Result<Pool<Postgres>, Error>>) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get one user query
            let result = sqlx::query_as::<_, User>(
                    "SELECT * FROM users WHERE user_id = $1"
                )
                .bind(user_id)
                .fetch_optional(pool)
                .await
                .map_err(|error| {
                    tracing::error!("failed to execute read_user query: {:?}", error);
                    error
                });

            match result {
                Ok(r) => HttpResponse::Ok().json(r),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
       },
        Err(error) => { 
            // no db was found
            tracing::error!("no database found: {:?}", error);
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}

// /users/{user_id}
#[instrument]
pub async fn update_user(_req: HttpRequest, user_id: Path<i32>, data_pool: Data<Result<Pool<Postgres>, Error>>, payload: Json<JsonUser>) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get one user query
            let option_user: Option<User> = sqlx::query_as::<_, User>(
                    "SELECT * FROM users WHERE user_id = $1"
                )
                .bind(&user_id)
                .fetch_optional(pool)
                .await
                .map_err(|error| {
                    tracing::error!("failed to execute read_user query: {:?}", error);
                    error
                })
                .unwrap();

            match option_user {
                Some(user) => {

                let jsonuser = payload.into_inner();

                let username  = if let Some(u) = jsonuser.username  { u } else { user.username };
                let password  = if let Some(p) = jsonuser.password  { p } else { user.password  };
                let email     = if let Some(e) = jsonuser.email     { e } else { user.email     };
                let discarded = if let Some(d) = jsonuser.discarded { d } else { user.discarded };

                // get all users query
                let result = sqlx::query(
                    "
                    UPDATE users SET 
                        username  = $1,
                        password  = $2,
                        email     = $3,
                        discarded = $4
                        WHERE user_id = $5
                    "
                    )
                    .bind(&username)
                    .bind(&password)
                    .bind(&email)
                    .bind(&discarded)
                    .bind(&user_id)
                    .execute(pool)
                    .await
                    .map_err(|error| {
                        tracing::error!("failed to execute update_user query: {:?}", error);
                        error
                    });

                    match result {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    }

                },
                None => {
                    // no user was found
                    tracing::error!("user_id: {} not found", &user_id);
                    HttpResponse::NotFound().finish()
                }
            }
        },
        Err(error) => { 
            // no db was found
            tracing::error!("no database found: {:?}", error);
            HttpResponse::ServiceUnavailable().finish()
        }
    }

}

// /users/{user_id}
#[instrument]
pub async fn delete_user(_req: HttpRequest, user_id: Path<i32>, data_pool: Data<Result<Pool<Postgres>, Error>>) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // delete map query
            let result_one = sqlx::query(
                    "
                    DELETE FROM user_role_map WHERE user_id = $1
                    "
                )
                .bind(user_id)
                .execute(pool)
                .await
                .map_err(|error| {
                    tracing::error!("failed to execute delete_role_map query: {:?}", error);
                    error
                });

                match result_one {
                    Ok(_) => {

                        // delete user query
                        let result_two = sqlx::query(
                                "
                                DELETE FROM users WHERE user_id = $1
                                "
                            )
                            .bind(user_id)
                            .execute(pool)
                            .await
                            .map_err(|error| {
                                tracing::error!("failed to execute delete_user query: {:?}", error);
                                HttpResponse::InternalServerError().finish()
                            });

                        match result_two {
                            Ok(_) => HttpResponse::Ok().finish(),
                            Err(_) => HttpResponse::InternalServerError().finish()
                        }

                    },
                    Err(_) => HttpResponse::InternalServerError().finish()
                }
        },
        Err(error) => { 
            // no db was found
            tracing::error!("no database found: {:?}", error);
            HttpResponse::ServiceUnavailable().finish()
        }
    }
}
