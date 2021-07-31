
use api::{config::get_config, server::server};
use std::net::TcpListener;
use sqlx::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // configure
    let configuration = get_config().expect("Failed to read configuration");
    let pool = Pool::connect(&configuration.database.connection_db_string())
        .await
        .expect("Failed to connect to database.");

    // bind
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    // run
    server(listener, pool)?.await
}
