use crate::modules::{api_config::api, website::website_config::website};
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
            .configure(api)
            .configure(website)
            .app_data(data_pool.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}


