use crate::apps::{admin::routes::*, users::routes::*};
use actix_web::{web, dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;
use sqlx::{Pool, Any, Error};
use std::net::TcpListener;

pub fn server(listener: TcpListener, result_pool: Result<Pool<Any>, Error>) -> Result<Server, Error> {

    // Wrap into data
    let data_pool = web::Data::new(result_pool);

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/status", web::get().to(status))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(read_users))
            .route("/users/{user_id}", web::get().to(read_user))
            .route("/users/{user_id}", web::put().to(update_user))
            .route("/users/{user_id}", web::delete().to(delete_user))
            .app_data(data_pool.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}
