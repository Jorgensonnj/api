use crate::{config::Settings, app::app_service_config::{front_end_service_config, back_end_service_config}};
use actix_web::{web, dev::Server, App, HttpServer};
//use rustls::ServerConfig;
use tracing_actix_web::TracingLogger;
use sqlx::{Pool, Postgres, Error};
use std::net::TcpListener;

pub fn server(
    listener: TcpListener,
    //_server_config: ServerConfig,
    result_pool: Result<Pool<Postgres>, Error>,
    configuration: Settings
) -> Result<Server, Error> {

    // Wrap into data
    let data_pool = web::Data::new(result_pool);
    let data_config = web::Data::new(configuration);

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(back_end_service_config)
            .configure(front_end_service_config)
            .app_data(data_pool.clone())
            .app_data(data_config.clone())
        }
    )
    //.listen_rustls(listener, server_config)?
    .listen(listener)?
    .run();

    Ok(server)
}
