use api::{
    config::get_config,
    server::server,
    //encryption::get_encryption,
    telemetry::{get_subscriber, init_subscriber}
};
use std::net::TcpListener;
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // configure
    let configuration = get_config().expect("Failed to read configuration");

    // log
    let subscriber = get_subscriber( &configuration.telemetry.env_filter, std::io::stdout );
    init_subscriber(subscriber);

    // encryption
    //let encryption = get_encryption( &configuration.server.key_path, &configuration.server.cert_path );

    // db connect if db exists
    let connection_url = &configuration.database
        .as_ref()
        .map_or_else(
            || "".to_string(),
            |database| database.connection_db_url()
    );

    let result_pool = Pool::<Postgres>::connect(&connection_url).await;

    // bind
    let address = format!("{}:{}", configuration.server.host, configuration.server.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    println!("\nListening at http://{} ...", listener.local_addr().unwrap() );

    // run
    server(listener, /* encryption, */ result_pool, configuration)
        .unwrap()
        .await
}
