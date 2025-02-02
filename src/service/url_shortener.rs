use crate::domain::{
    errors::UrlShortenerError,
    models::{LinkStats, LongUrl, ShortLink, Slug},
};
use crate::infrastructure::repositories::in_memory::InMemoryRepository;
use rand::distr::Alphanumeric;
use rand::Rng;
use std::sync::Mutex;

pub struct UrlShortenerService {
    repository: Mutex<InMemoryRepository>,
}

impl UrlShortenerService {
    pub fn new(repository: InMemoryRepository) -> Self {
        Self {
            repository: Mutex::new(repository),
        }
    }

    pub fn create_short_link(
        &self,
        url: LongUrl,
        custom_slug: Option<Slug>,
    ) -> Result<ShortLink, UrlShortenerError> {
        let mut repository = self.repository.lock().unwrap();

        if url.0.is_empty() {
            return Err(UrlShortenerError::InvalidUrl);
        }

        let slug = match custom_slug {
            Some(slug) => {
                if repository.exists(&slug) {
                    return Err(UrlShortenerError::SlugConflict);
                }
                slug
            }
            None => Slug(self.generate_unique_slug(&repository)),
        };

        let short_link = ShortLink {
            slug: slug.clone(),
            url: url.clone(),
        };
        repository.save(&short_link)?;

        Ok(short_link)
    }

    pub fn redirect(&self, slug: &Slug) -> Result<LongUrl, UrlShortenerError> {
        let mut repository = self.repository.lock().unwrap();
        let short_link = repository.find_by_slug(slug)?;
        repository.increment_redirects(slug)?;

        Ok(short_link.url)
    }

    pub fn get_stats(&self, slug: &Slug) -> Result<LinkStats, UrlShortenerError> {
        let repository = self.repository.lock().unwrap();
        let link = repository.find_by_slug(slug)?;
        let redirect_count = repository.get_redirect_count(slug)?;

        Ok(LinkStats {
            link,
            redirect_count,
        })
    }

    fn generate_unique_slug(&self, repository: &InMemoryRepository) -> String {
        loop {
            let slug = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>();

            if !repository.exists(&Slug(slug.clone())) {
                return slug;
            }
        }
    }
}
