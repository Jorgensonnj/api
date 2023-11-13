use actix_web::{web::{Json, Data, Path, Query}, HttpRequest, HttpResponse, Responder};
use super::super::{
    actions,
    super::super::shared_module::api::models::*
};
use sqlx::{Pool, Postgres, Error};
use tracing::instrument;
use std::collections::HashMap;

// /users
#[instrument]
pub async fn create_user(
    _req: HttpRequest,
    data_pool: Data<Result<Pool<Postgres>, Error>>,
    payload: Json<JsonUser>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    // is there a db connection?
    let pool = _get_pool(result_pool).await.unwrap();

    let create_result = actions::create(pool, &payload).await;

    match create_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// /users
#[instrument]
pub async fn read_users(
    req: HttpRequest,
    data_pool: Data<Result<Pool<Postgres>, Error>>
) -> impl Responder {
    // unwrap
    let result_pool = data_pool.get_ref();

    let query_map = match Query::<HashMap<String, String>>::from_query(req.query_string()) {
        Ok(query) => query.into_inner(),
        Err(_) => HashMap::<String, String>::new()
    };

    // is there a db connection?
    let pool = _get_pool(result_pool).await.unwrap();

    // get all users
    let read_result = actions::read_all(pool, &query_map).await;

    match read_result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish()
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

    // is there a db connection?
    let pool = _get_pool(result_pool).await.unwrap();

    // get user
    let read_result = actions::read_one(pool, &user_id).await;

    match read_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
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

    // is there a db connection?
    let pool = _get_pool(result_pool).await.unwrap();

    // get one user
    let read_result = actions::read_one(pool, &user_id).await;

    let user_vector = match read_result {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish()
    };

    let mut json_user = payload.into_inner();

    for user in user_vector.iter() {
        json_user.username  = if let Some(u) = json_user.username  { Some(u) } else { Some(user.username.clone())  };
        json_user.password  = if let Some(p) = json_user.password  { Some(p) } else { Some(user.password.clone())  };
        json_user.email     = if let Some(e) = json_user.email     { Some(e) } else { Some(user.email.clone())     };
        json_user.discarded = if let Some(d) = json_user.discarded { Some(d) } else { Some(user.discarded)         };
    }

    // update user
    let update_result = actions::update(pool, user_id, json_user).await;

    match update_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish()
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

    // is there a db connection?
    let pool = _get_pool(result_pool).await.unwrap();

    // delete user
    let delete_result = actions::delete(pool, user_id).await;

    match delete_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// helper function
async fn _get_pool(
    result_pool: &Result<Pool<Postgres>, Error>
) -> Result<&Pool<Postgres>, HttpResponse> {
    match result_pool {
        // no db was found
        Ok(pool) => Ok(pool),
        Err(error) => { 
            tracing::error!("no database found: {:?}", error);
            Err(HttpResponse::ServiceUnavailable().finish())
        },
    }
}
