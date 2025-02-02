use crate::{LongUrl, ShortLink, ShortenerError, Slug};
use bincode;
use serde::{Deserialize, Serialize};
use sled::Db;

#[derive(Clone)]
pub struct UrlRepository {
    db: Db,
}

impl UrlRepository {
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn save(&self, url: LongUrl, slug: Option<Slug>) -> Result<ShortLink, ShortenerError> {
        let slug = slug.unwrap_or_else(|| Slug("random_slug".to_string()));

        if self.db.get(&slug.0)?.is_some() {
            return Err(ShortenerError::SlugAlreadyInUse);
        }

        let serialized = bincode::serialize(&url).map_err(|_| ShortenerError::StorageError)?;
        self.db.insert(slug.0.clone(), serialized)?;
        self.db.flush().map_err(|_| ShortenerError::StorageError)?;

        Ok(ShortLink { slug, url })
    }

    pub fn find_by_slug(&self, slug: &Slug) -> Result<LongUrl, ShortenerError> {
        match self.db.get(&slug.0)? {
            Some(ivec) => {
                let url: LongUrl =
                    bincode::deserialize(&ivec).map_err(|_| ShortenerError::StorageError)?;
                Ok(url)
            }
            None => Err(ShortenerError::SlugNotFound),
        }
    }
}
