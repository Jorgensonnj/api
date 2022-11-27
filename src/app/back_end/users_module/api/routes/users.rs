use actix_web::{web::{Json, Data, Path}, HttpRequest, HttpResponse, Responder};
use super::super::{ models::users::JsonUser, actions};
use sqlx::{Pool, Postgres, Error};
use tracing::instrument;

// /users
#[instrument]
pub async fn create_user(
    _req: HttpRequest,
    data_pool: Data<Result<Pool<Postgres>, Error>>,
    payload: Json<JsonUser>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // create user
            let create_result = actions::create(pool, &payload).await;

            match create_result {
                Ok(user) => HttpResponse::Ok().json(user),
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
pub async fn read_users(
    _req: HttpRequest,
    data_pool: Data<Result<Pool<Postgres>, Error>>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get all users
            let read_result = actions::read_all(pool).await;

            match read_result {
                Ok(users) => HttpResponse::Ok().json(users),
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
pub async fn read_user(
    _req: HttpRequest,
    user_id: Path<i32>,
    data_pool: Data<Result<Pool<Postgres>, Error>>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get user
            let read_result = actions::read_one(pool, user_id).await;

            match read_result {
                Ok(user) => HttpResponse::Ok().json(user),
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
pub async fn update_user(
    _req: HttpRequest,
    user_id: Path<i32>,
    data_pool: Data<Result<Pool<Postgres>, Error>>,
    payload: Json<JsonUser>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get one user
            let option_user = actions::read_one(pool, user_id).await;

            match option_user {
                Ok(user) => {

                let mut json_user = payload.into_inner();

                json_user.username  =
                    if let Some(u) = json_user.username  { Some(u) } else { Some(user.username)  };
                json_user.password  =
                    if let Some(p) = json_user.password  { Some(p) } else { Some(user.password)  };
                json_user.email     =
                    if let Some(e) = json_user.email     { Some(e) } else { Some(user.email)     };
                json_user.discarded =
                    if let Some(d) = json_user.discarded { Some(d) } else { Some(user.discarded) };

                // update user
                let update_result = actions::update(pool, user_id, json_user).await;

                    match update_result {
                        Ok(user) => HttpResponse::Ok().json(user),
                        Err(_) => HttpResponse::InternalServerError().finish()
                    }
                },
                Err(_) => {
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
pub async fn delete_user(
    _req: HttpRequest,
    user_id: Path<i32>,
    data_pool: Data<Result<Pool<Postgres>, Error>>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();
    let user_id     = user_id.into_inner();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // delete user
            let delete_result = actions::delete(pool, user_id).await;

            match delete_result {
                Ok(_) => HttpResponse::Ok().finish(),
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
