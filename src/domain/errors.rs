use thiserror::Error;

#[derive(Error, Debug)]
pub enum UrlShortenerError {
    #[error("Invalid URL")]
    InvalidUrl,

    #[error("Slug already exists")]
    SlugConflict,

    #[error("Slug not found")]
    SlugNotFound,
}
