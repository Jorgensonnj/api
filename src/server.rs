use crate::apps::admin::routes::*;
use actix_web::{web, dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;
use sqlx::{Pool, Postgres};
use std::net::TcpListener;
use std::io::Error;

pub fn server(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, Error> {
    let pool = web::Data::new(pool);

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/status", web::get().to(status))
            .app_data(pool.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}
