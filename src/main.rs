use api::{
    config::get_config,
    server::server,
    encryption::get_encryption,
    telemetry::{get_subscriber, init_subscriber}
};
use std::net::TcpListener;
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // configure
    let configuration = get_config().expect("Failed to read configuration");

    // log
    let subscriber = get_subscriber(
        configuration.telemetry.env_filter,
        std::io::stdout
    );
    init_subscriber(subscriber);

    // encryption
    let server_config = get_encryption(
        configuration.server.key_path,
        configuration.server.cert_path,
    );

    // DB connect
    let result_pool = Pool::<Postgres>::connect(
        &configuration.database.connection_db_string()
    ).await;

    // bind
    let address = format!("0.0.0.0:{}", configuration.server.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    println!("\nListening at http://{} ...", listener.local_addr().unwrap() );

    // run
    server(listener, server_config, result_pool)
        .unwrap()
        .await
}
