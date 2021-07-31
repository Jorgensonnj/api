
use crate::apps::admin::routes::*;
use actix_web::{web, dev::Server, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;
use std::io::Error;

pub fn server(listener: TcpListener, pool: Pool<Postgres>) -> Result<Server, Error> {

    let pool = web::Data::new(pool);
    let address = listener.local_addr().unwrap();

    // Initalize server
    let server = HttpServer::new(move || {
        App::new()
            .route("/status", web::get().to(status))
            .app_data(pool.clone())
        }
    )
    .listen(listener)?
    .run();

    println!("\nRunning at http://{}", address);

    Ok(server)
}
