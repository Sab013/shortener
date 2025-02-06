#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    shortener::server::run_server().await
}
