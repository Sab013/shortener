use super::handlers;
use crate::api::docs::ApiDoc;
use crate::ShortenerService;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(service: Arc<ShortenerService>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest(
            "/api/v1/links",
            Router::new()
                .route("/slug", post(handlers::create_short_link))
                .route("/{slug}/redirect", get(handlers::redirect))
                .route("/{slug}/stats", get(handlers::get_stats))
                .with_state(service),
        )
}
