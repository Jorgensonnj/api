use api::{config::get_config, server::server, telemetry::{get_subscriber, init_subscriber}};
use std::net::TcpListener;
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // log
    // Options: info, error
    let subscriber = get_subscriber("error".into(), std::io::stdout);
    init_subscriber(subscriber);

    // configure
    let configuration = get_config().expect("Failed to read configuration");

    // DB connect
    let result_pool = Pool::<Postgres>::connect(&configuration.database.connection_db_string()).await;

    // bind
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    println!("\nListening at http://{} ...", listener.local_addr().unwrap() );

    // run
    server(listener, result_pool)
        .unwrap()
        .await
}
