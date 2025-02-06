use super::handlers;
use crate::api::docs::ApiDoc;
use crate::ShortenerService;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub fn create_router(service: Arc<ShortenerService>) -> Router {
    Router::new()
        .route("/slug", post(handlers::create_short_link))
        .route("/{slug}/redirect", get(handlers::redirect))
        .route("/{slug}/stats", get(handlers::get_stats))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .with_state(service)
}
