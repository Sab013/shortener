use super::dto::{CreateLinkRequest, CreateLinkResponse};
use crate::domain::models::{LongUrl, Slug};
use crate::domain::LinkStats;
use crate::service::shortener_service::ShortenerService;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::{info, warn};

const BRAND_URL: &str = "http://brand.url/";
const LOCATION: &str = "location";

#[utoipa::path(
    post,
    path = "/api/v1/links/slug",
    request_body = CreateLinkRequest,
    responses(
        (status = 201, description = "Short link created successfully", body = CreateLinkResponse),
        (status = 400, description = "Bad request")
    ),
    tags = ["shortener"]
)]
pub async fn create_short_link(
    State(service): State<Arc<ShortenerService>>,
    Json(req): Json<CreateLinkRequest>,
) -> impl IntoResponse {
    let url = LongUrl(req.url);
    let slug = req.slug.map(|s| Slug(s));

    match service.create_short_link(url.clone(), slug).await {
        Ok(short_link) => (
            StatusCode::CREATED,
            Json(CreateLinkResponse {
                short_url: format!("{}{}", BRAND_URL, short_link.slug.0),
                original_url: short_link.url.0,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/links/{slug}/redirect",
    params(
        ("slug" = String, Path, description = "Short URL slug", example = "vasya-999")
    ),
    responses(
        (status = 307, description = "Permanent redirect to original URL"),
        (status = 404, description = "Short link not found")
    ),
    tags = ["shortener"]
)]
pub async fn redirect(
    State(service): State<Arc<ShortenerService>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    info!("🔍 Запрос на редирект: {}", slug);
    match service.redirect(&Slug(slug)).await {
        Ok(url) => {
            info!("✅ Redirect to: {}", url.0);
            let mut headers = HeaderMap::new();
            headers.insert(LOCATION, url.0.parse().unwrap());
            (StatusCode::TEMPORARY_REDIRECT, headers).into_response()
        }
        Err(e) => {
            warn!("❌ Redirect error: {}", e);
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/links/{slug}/stats",
    params(
        ("slug" = String, Path, description = "Short URL slug", example = "vasya-999")
    ),
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = LinkStats),
        (status = 404, description = "Short link not found")
    ),
    tags = ["shortener"]
)]
pub async fn get_stats(
    State(service): State<Arc<ShortenerService>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match service.get_stats(&Slug(slug)).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}
