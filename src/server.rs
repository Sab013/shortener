use crate::ShortenerService;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use crate::api::routes;
use crate::config::AppConfig;
use crate::infrastructure::storage::redis::shortener_repository::ShortenerRepository;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("🚀 Initializing Redis Repository...");
    let repository = Arc::new(
        ShortenerRepository::new(&config.redis_url)
            .await
            .map_err(|e| anyhow::Error::msg(format!("Failed to initialize repository: {}", e)))?,
    );

    let service = Arc::new(ShortenerService::new(repository));

    // CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Middleware stack
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Router
    let app = Router::new()
        .merge(routes::create_router(service))
        .layer(middleware_stack);

    // Server binding
    let addr = format!("0.0.0.0:{}", config.server_port).parse::<SocketAddr>()?;

    info!("🚀 Server running on http://{}", addr);
    info!("📚 Swagger UI available at http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
