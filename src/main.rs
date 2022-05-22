pub(crate) use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
