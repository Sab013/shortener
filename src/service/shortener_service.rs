use crate::domain::{
    errors::DomainErrors,
    models::{LinkStats, LongUrl, ShortLink, Slug},
};
use crate::infrastructure::storage::redis::shortener_repository::ShortenerRepository;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::sync::Arc;
use tracing::info;

pub struct ShortenerService {
    repository: Arc<ShortenerRepository>,
}

impl ShortenerService {
    pub fn new(repository: Arc<ShortenerRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_short_link(
        &self,
        url: LongUrl,
        custom_slug: Option<Slug>,
    ) -> Result<ShortLink, DomainErrors> {
        if url.0.is_empty() {
            return Err(DomainErrors::InvalidUrl);
        }

        let slug = match custom_slug {
            Some(slug) => {
                if self.repository.exists(&slug).await? {
                    return Err(DomainErrors::SlugConflict);
                }
                slug
            }
            None => Slug(self.generate_unique_slug().await),
        };

        let short_link = ShortLink {
            slug: slug.clone(),
            url: url.clone(),
        };

        self.repository.save(&short_link).await?;

        Ok(short_link)
    }

    pub async fn redirect(&self, slug: &Slug) -> Result<LongUrl, DomainErrors> {
        info!("Redirect requested for slug: {}", slug.0);
        let short_link = self.repository.find_by_slug(slug).await?;
        info!("Found short link: {:?}", short_link);
        self.repository.increment_redirects(slug).await?;
        info!("Redirect count incremented for {}", slug.0);
        Ok(short_link.url)
    }

    pub async fn get_stats(&self, slug: &Slug) -> Result<LinkStats, DomainErrors> {
        let link = self.repository.find_by_slug(slug).await?;
        let redirect_count = self.repository.get_redirect_count(slug).await?;
        Ok(LinkStats {
            link,
            redirect_count,
        })
    }

    async fn generate_unique_slug(&self) -> String {
        loop {
            let slug: String = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();

            if !self
                .repository
                .exists(&Slug(slug.clone()))
                .await
                .unwrap_or(false)
            {
                return slug;
            }
        }
    }
}
