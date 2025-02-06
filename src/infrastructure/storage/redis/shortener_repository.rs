use super::error_utils::RedisResultExt;
use crate::domain::{
    errors::DomainErrors,
    models::{ShortLink, Slug},
};
use crate::models::LongUrl;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct ShortenerRepository {
    connection: Arc<MultiplexedConnection>,
}

impl ShortenerRepository {
    const REDIRECTS_KEY_PREFIX: &'static str = "redirects:";

    pub async fn new(redis_url: &str) -> Result<Self, DomainErrors> {
        info!(
            "Initializing Shortener repository with Redis URL: {}",
            redis_url
        );
        let client = Client::open(redis_url).map_storage_err()?;

        let connection = client
            .get_multiplexed_async_connection()
            .await
            .map_storage_err()?;

        info!("Successfully connected to Redis");
        Ok(Self {
            connection: Arc::new(connection),
        })
    }

    fn get_connection(&self) -> MultiplexedConnection {
        self.connection.as_ref().clone()
    }

    fn get_redirects_key(slug: &Slug) -> String {
        format!("{}{}", Self::REDIRECTS_KEY_PREFIX, slug.0)
    }

    pub async fn exists(&self, slug: &Slug) -> Result<bool, DomainErrors> {
        info!("Checking existence of slug: {}", slug.0);
        let exists = self
            .get_connection()
            .exists(&slug.0)
            .await
            .map_storage_err()?;

        info!(
            "Slug {} {}",
            slug.0,
            if exists { "exists" } else { "does not exist" }
        );
        Ok(exists)
    }

    pub async fn save(&self, short_link: &ShortLink) -> Result<(), DomainErrors> {
        info!(
            "Saving short link: {} -> {}",
            short_link.slug.0, short_link.url.0
        );
        let mut conn = self.get_connection();
        let mut pipe = redis::pipe();

        pipe.set(&short_link.slug.0, &short_link.url.0)
            .set(Self::get_redirects_key(&short_link.slug), 0);

        pipe.exec_async(&mut conn).await.map_storage_err()?;

        info!("Successfully saved short link: {}", short_link.slug.0);
        Ok(())
    }

    pub async fn find_by_slug(&self, slug: &Slug) -> Result<ShortLink, DomainErrors> {
        info!("Looking up URL for slug: {}", slug.0);
        let url = self
            .get_connection()
            .get::<_, String>(&slug.0)
            .await
            .map_storage_err()
            .map(|url| ShortLink {
                slug: slug.clone(),
                url: LongUrl(url),
            })?;

        info!("Found URL for slug {}: {}", slug.0, url.url.0);
        Ok(url)
    }

    pub async fn increment_redirects(&self, slug: &Slug) -> Result<(), DomainErrors> {
        info!("Incrementing redirect count for slug: {}", slug.0);
        let new_count: i64 = self
            .get_connection()
            .incr(Self::get_redirects_key(slug), 1)
            .await
            .map_storage_err()?;

        info!("Redirect count updated: {} -> {}", slug.0, new_count);
        Ok(())
    }

    pub async fn get_redirect_count(&self, slug: &Slug) -> Result<u64, DomainErrors> {
        info!("Fetching redirect count for slug: {}", slug.0);
        let count = self
            .get_connection()
            .get::<_, u64>(Self::get_redirects_key(slug))
            .await
            .map_storage_err()?;

        info!("Current redirect count for {}: {}", slug.0, count);
        Ok(count)
    }
}
