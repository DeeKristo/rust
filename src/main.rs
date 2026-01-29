use zero2prod::run;
use std::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    // run("127.0.0.1:8000")?.await
    let listener= TcpListener::bind("127.0.0.1:0")
.expect("Failed to bind random port");
    run(listener)?.await
}
