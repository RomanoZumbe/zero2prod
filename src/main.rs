use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    let _ = zero2prod::run(listener).expect("Failed to bind address");
    Ok(())
}
