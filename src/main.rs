use std::net::TcpListener;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string()).await
        .expect("Failed to connect to postgres.");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    let _ = run(listener, connection).expect("Failed to bind address").await;
    Ok(())
}
