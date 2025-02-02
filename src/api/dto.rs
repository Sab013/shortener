use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateLinkRequest {
    #[schema(example = " https://www.google.com", format = "uri")]
    pub url: String,
    #[schema(example = "vasya-999")]
    pub slug: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CreateLinkResponse {
    #[schema(example = "http://short.url/vasya-999")]
    pub short_url: String,
    #[schema(example = " https://www.google.com")]
    pub original_url: String,
}
