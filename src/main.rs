#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    url_shortener::run_server().await
}
