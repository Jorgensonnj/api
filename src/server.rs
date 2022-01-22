use crate::modules::{admin::api::routes::*, users::api::routes::*};
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
            .route("/status", web::get().to(status))
            .service(
                web::scope("/users")
                    .service(
                        web::resource("")
                            .route(web::post().to(create_user))
                            .route(web::get().to(read_users))
                    )
                    .service(
                        web::resource("/{user_id}")
                            .route(web::get().to(read_user))
                            .route(web::put().to(update_user))
                            .route(web::delete().to(delete_user))
                    )
            )
            .app_data(data_pool.clone())
        }
    )
    .listen(listener)?
    .run();

    Ok(server)
}


