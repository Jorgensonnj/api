use crate::app::app_config::{front_end_config, back_end_config};
use actix_web::{web, dev::Server, App, HttpServer};
use rustls::ServerConfig;
use tracing_actix_web::TracingLogger;
use sqlx::{Pool, Postgres, Error};
use std::net::TcpListener;

pub fn server(
    listener: TcpListener,
    server_config: ServerConfig,
    result_pool: Result<Pool<Postgres>, Error>
) -> Result<Server, Error> {

    // Wrap into data
    let data_pool = web::Data::new(result_pool);

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(back_end_config)
            .configure(front_end_config)
            .app_data(data_pool.clone())
        }
    )
    .listen_rustls(listener, server_config)?
    .run();

    Ok(server)
}
