use crate::UrlShortenerService;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::docs::ApiDoc;
use crate::api::routes;
use crate::config::AppConfig;
use crate::infrastructure::storage::redis::shortened_urls::ShortLinkRepository;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
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
