use crate::domain::{
    errors::UrlShortenerError,
    models::{ShortLink, Slug},
};
use crate::models::LongUrl;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct UrlRepository {
    connection: Arc<MultiplexedConnection>,
}

impl UrlRepository {
    pub async fn new(redis_url: &str) -> Result<Self, UrlShortenerError> {
        let client = Client::open(redis_url).map_err(|_| UrlShortenerError::StorageError)?;

        let connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| UrlShortenerError::StorageError)?;

        Ok(Self {
            connection: Arc::new(connection),
        })
    }

    pub async fn exists(&self, slug: &Slug) -> Result<bool, UrlShortenerError> {
        let mut conn = self.connection.as_ref().clone();
        conn.exists(&slug.0)
            .await
            .map_err(|_| UrlShortenerError::StorageError)
    }

    pub async fn save(&self, short_link: &ShortLink) -> Result<(), UrlShortenerError> {
        let mut conn = self.connection.as_ref().clone();

        let _: () = conn
            .set(&short_link.slug.0, &short_link.url.0)
            .await
            .map_err(|_| UrlShortenerError::StorageError)?;

        let _: () = conn
            .set(format!("redirects:{}", short_link.slug.0), 0)
            .await
            .map_err(|_| UrlShortenerError::StorageError)?;

        Ok(())
    }

    pub async fn find_by_slug(&self, slug: &Slug) -> Result<ShortLink, UrlShortenerError> {
        let mut conn = self.connection.as_ref().clone();

        match conn.get::<_, String>(&slug.0).await {
            Ok(url) => Ok(ShortLink {
                slug: slug.clone(),
                url: LongUrl(url),
            }),
            Err(_) => Err(UrlShortenerError::SlugNotFound),
        }
    }

    pub async fn increment_redirects(&self, slug: &Slug) -> Result<(), UrlShortenerError> {
        let mut conn = self.connection.as_ref().clone();

        let new_count: i64 = conn
            .incr(format!("redirects:{}", slug.0), 1)
            .await
            .map_err(|_| UrlShortenerError::StorageError)?;

        info!("Redirect count updated: {} -> {}", slug.0, new_count);

        Ok(())
    }

    pub async fn get_redirect_count(&self, slug: &Slug) -> Result<u64, UrlShortenerError> {
        let mut conn = self.connection.as_ref().clone();
        match conn.get::<_, u64>(format!("redirects:{}", slug.0)).await {
            Ok(count) => Ok(count),
            Err(_) => Err(UrlShortenerError::SlugNotFound),
        }
    }
}
