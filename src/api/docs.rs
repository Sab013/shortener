use crate::api::{
    dto::{CreateLinkRequest, CreateLinkResponse},
    handlers,
};
use crate::domain::LinkStats;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::create_short_link,
        handlers::redirect,
        handlers::get_stats
    ),
    components(
        schemas(
            CreateLinkRequest,
            CreateLinkResponse,
            LinkStats,
        )
    ),
    info(
        title = "URL Shortener API",
        version = "1.0.0",
        description = "API для сокращения URL-адресов",
        contact(
            name = "Vitaly Vasiltsov",
            email = "dev9900195@gmail.com",
        )
    ),
    tags(
        (name = "shortener", description = "URL Shortener endpoints")
    )
)]
pub struct ApiDoc;
