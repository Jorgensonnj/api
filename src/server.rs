use crate::modules::{admin::api::config::admin_api, users::api::config::user_api};
use actix_web::{web, dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;
use sqlx::{Pool, Postgres, Error};
use std::net::TcpListener;

pub fn server(listener: TcpListener, result_pool: Result<Pool<Postgres>, Error>) -> Result<Server, Error> {

    // Wrap into data
    let data_pool = web::Data::new(result_pool);

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(admin_api)
            .configure(user_api)
            .app_data(data_pool.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}


