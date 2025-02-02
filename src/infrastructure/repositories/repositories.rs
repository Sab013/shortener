use crate::{LongUrl, ShortLink, ShortenerError, Slug};
use std::collections::HashMap;

pub struct UrlRepository {
    links: HashMap<String, LongUrl>,
}

impl UrlRepository {
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
        }
    }

    pub fn save(&mut self, url: LongUrl, slug: Option<Slug>) -> Result<ShortLink, ShortenerError> {
        let slug = slug.unwrap_or_else(|| Slug("random_slug".to_string()));
        if self.links.contains_key(&slug.0) {
            return Err(ShortenerError::SlugAlreadyInUse);
        }
        self.links.insert(slug.0.clone(), url.clone());
        Ok(ShortLink { slug, url })
    }
}
