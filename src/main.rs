use actix_web::{web, App, HttpServer};
use url_shortener::{
    api::{docs::ApiDoc, routes}, // импортируйте ApiDoc
    infrastructure::repositories::in_memory::InMemoryRepository,
    service::url_shortener::UrlShortenerService,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let repository = InMemoryRepository::new();
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
    .await
}
