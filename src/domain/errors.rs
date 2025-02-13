use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainErrors {
    #[error("Invalid URL")]
    InvalidUrl,

    #[error("Slug already exists")]
    SlugConflict,

    #[error("Slug not found")]
    SlugNotFound,

    #[error("Storage error")]
    StorageError,
}
