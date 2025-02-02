use std::collections::HashMap;

use crate::domain::{
    errors::UrlShortenerError,
    models::{ShortLink, Slug},
};

pub struct InMemoryRepository {
    links: HashMap<Slug, ShortLink>,
    redirects: HashMap<Slug, u64>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
            redirects: HashMap::new(),
        }
    }

    pub fn exists(&self, slug: &Slug) -> bool {
        self.links.contains_key(slug)
    }

    pub fn save(&mut self, short_link: &ShortLink) -> Result<(), UrlShortenerError> {
        self.links
            .insert(short_link.slug.clone(), short_link.clone());
        self.redirects.entry(short_link.slug.clone()).or_insert(0);
        Ok(())
    }

    pub fn find_by_slug(&self, slug: &Slug) -> Result<ShortLink, UrlShortenerError> {
        self.links
            .get(slug)
            .cloned()
            .ok_or(UrlShortenerError::SlugNotFound)
    }

    pub fn increment_redirects(&mut self, slug: &Slug) -> Result<(), UrlShortenerError> {
        match self.redirects.get_mut(slug) {
            Some(count) => {
                *count += 1;
                Ok(())
            }
            None => Err(UrlShortenerError::SlugNotFound),
        }
    }

    pub fn get_redirect_count(&self, slug: &Slug) -> Result<u64, UrlShortenerError> {
        self.redirects
            .get(slug)
            .copied()
            .ok_or(UrlShortenerError::SlugNotFound)
    }
}
