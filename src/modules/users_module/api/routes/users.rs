use actix_web::{web::{Json, Data, Path}, HttpRequest, HttpResponse, Responder};
use crate::modules::users_module::api::models::{users, users::JsonUser};
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

            // get all users query
            let result = users::create(pool, payload.into_inner()).await;

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
pub async fn read_users(
    _req: HttpRequest,
    data_pool: Data<Result<Pool<Postgres>, Error>>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection
    match result_pool {
        Ok(pool) => {

            // get all users query
            let result = users::read_all(pool).await;

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

            // get one user query
            let result = users::read_one(pool, user_id).await;

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

            // get one user query
            let option_user = users::read_one(pool, user_id).await;

            match option_user {
                Ok(user) => {

                let mut jsonuser = payload.into_inner();

                jsonuser.username  =
                    if let Some(u) = jsonuser.username  { Some(u) } else { Some(user.username)  };
                jsonuser.password  =
                    if let Some(p) = jsonuser.password  { Some(p) } else { Some(user.password)  };
                jsonuser.email     =
                    if let Some(e) = jsonuser.email     { Some(e) } else { Some(user.email)     };
                jsonuser.discarded =
                    if let Some(d) = jsonuser.discarded { Some(d) } else { Some(user.discarded) };

                // get all users query
                let result = users::update(pool, user_id, jsonuser).await;

                    match result {
                        Ok(_) => HttpResponse::Ok().finish(),
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

            // delete map query
            let result_one = users::delete(pool, user_id).await;

            match result_one {
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
