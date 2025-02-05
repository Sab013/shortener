use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing::info;
use url_shortener::api::docs::ApiDoc;
use url_shortener::api::routes;
use url_shortener::config::AppConfig;
use url_shortener::UrlShortenerService;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use url_shortener::infrastructure::storage::redis::shortened_urls::ShortLinkRepository;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("🚀 Initializing Redis Repository...");
    let repository = Arc::new(
        ShortLinkRepository::new(&config.redis_url)
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to initialize repository: {}", e)))?,
    );

    let service = web::Data::new(UrlShortenerService::new(repository));

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(service.clone())
            .configure(routes::config)
    })
    .bind(("0.0.0.0", config.server_port))?
    .run()
    .await?;

    Ok(())
}
