use super::dto::{CreateLinkRequest, CreateLinkResponse};
use crate::domain::models::{LongUrl, Slug};
use crate::domain::LinkStats;
use crate::service::url_shortener::UrlShortenerService;
use actix_web::{web, HttpResponse, Responder};

const BRAND_URL: &str = "http://brand.url/";
const LOCATION: &str = "Location";

#[utoipa::path(
    post,
    path = "/api/v1/links/slug",
    request_body = CreateLinkRequest,
    summary = "Create slug from a link",
    responses(
        (status = 201, description = "Short link created successfully", body = CreateLinkResponse),
        (status = 400, description = "Bad request")
    ),
    tags = ["shortener"]
)]
pub async fn create_short_link(
    service: web::Data<UrlShortenerService>,
    req: web::Json<CreateLinkRequest>,
) -> impl Responder {
    let url = LongUrl(req.url.clone());
    let slug = req.slug.as_ref().map(|s| Slug(s.to_string()));

    match service.create_short_link(url.clone(), slug) {
        Ok(short_link) => HttpResponse::Created().json(CreateLinkResponse {
            short_url: format!("{}{}", BRAND_URL, short_link.slug.0),
            original_url: short_link.url.0,
        }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/links/{slug}/redirect",
    params(
        ("slug" = String, Path, description = "Short URL slug", example = "vasya-999")
    ),
    summary = "Redirecting requests using slug to original URL",
    responses(
        (status = 307, description = "Permanent redirect to original URL. Note: This endpoint \
        performs a redirect which may not work properly in Swagger UI. Please use a direct HTTP \
        client like curl or Postman to test redirects."),
        (status = 404, description = "Short link not found")
    ),
    tags = ["shortener"]
)]
pub async fn redirect(
    service: web::Data<UrlShortenerService>,
    slug: web::Path<String>,
) -> impl Responder {
    match service.redirect(&Slug(slug.into_inner())) {
        Ok(url) => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, url.0))
            .finish(),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/links/{slug}/stats",
    params(
        ("slug" = String, Path, description = "Short URL slug", example = "vasya-999")
    ),
    summary = "Get slug stats",
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = LinkStats),
        (status = 404, description = "Short link not found")
    ),
    tags = ["shortener"]
)]
pub async fn get_stats(
    service: web::Data<UrlShortenerService>,
    slug: web::Path<String>,
) -> impl Responder {
    match service.get_stats(&Slug(slug.into_inner())) {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
