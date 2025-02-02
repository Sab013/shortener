use actix_web::{web, App, HttpServer};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use url_shortener::{
    api::{docs::ApiDoc, routes},
    infrastructure::repositories::persistent::PersistentRepository,
    service::url_shortener::UrlShortenerService,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<()> {
    let repository = Arc::new(Mutex::new(
        PersistentRepository::new("url_shortener_db")
            .map_err(|e| anyhow::Error::msg(format!("Failed to initialize repository: {}", e)))?,
    ));

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
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}