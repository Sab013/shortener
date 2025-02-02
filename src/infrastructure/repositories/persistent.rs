use crate::domain::{
    errors::UrlShortenerError,
    models::{ShortLink, Slug},
};
use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};
use sled::Db;

#[derive(Clone)]
pub struct PersistentRepository {
    db: Db,
}

impl PersistentRepository {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn exists(&self, slug: &Slug) -> bool {
        self.db
            .get(&slug.0)
            .map(|res| res.is_some())
            .unwrap_or(false)
    }

    pub fn save(&self, short_link: &ShortLink) -> Result<(), UrlShortenerError> {
        let serialized =
            encode_to_vec(short_link, standard()).map_err(|_| UrlShortenerError::StorageError)?;

        self.db
            .insert(short_link.slug.0.clone(), serialized.to_vec()) // Исправлено
            .map_err(|_| UrlShortenerError::StorageError)?;

        self.db
            .insert(
                format!("redirects:{}", short_link.slug.0),
                0u64.to_be_bytes().to_vec(), // Исправлено
            )
            .map_err(|_| UrlShortenerError::StorageError)?;

        self.db
            .flush()
            .map_err(|_| UrlShortenerError::StorageError)?;

        Ok(())
    }

    pub fn find_by_slug(&self, slug: &Slug) -> Result<ShortLink, UrlShortenerError> {
        match self
            .db
            .get(&slug.0)
            .map_err(|_| UrlShortenerError::StorageError)?
        {
            Some(value) => {
                let (short_link, _): (ShortLink, _) = decode_from_slice(&value, standard())
                    .map_err(|_| UrlShortenerError::StorageError)?;
                Ok(short_link)
            }
            None => Err(UrlShortenerError::SlugNotFound),
        }
    }

    pub fn increment_redirects(&self, slug: &Slug) -> Result<(), UrlShortenerError> {
        let key = format!("redirects:{}", slug.0);

        let current_count = self
            .db
            .get(&key)
            .map_err(|_| UrlShortenerError::StorageError)?
            .map(|v| {
                let mut buf = [0u8; 8];
                if v.len() == 8 {
                    buf.copy_from_slice(&v);
                }
                u64::from_be_bytes(buf)
            })
            .unwrap_or(0);

        self.db
            .insert(key, (current_count + 1).to_be_bytes().to_vec()) // Исправлено
            .map_err(|_| UrlShortenerError::StorageError)?;

        self.db
            .flush()
            .map_err(|_| UrlShortenerError::StorageError)?;

        Ok(())
    }

    pub fn get_redirect_count(&self, slug: &Slug) -> Result<u64, UrlShortenerError> {
        let key = format!("redirects:{}", slug.0);
        match self
            .db
            .get(&key)
            .map_err(|_| UrlShortenerError::StorageError)?
        {
            Some(v) => {
                if v.len() == 8 {
                    Ok(u64::from_be_bytes(v.as_ref().try_into().unwrap()))
                } else {
                    Err(UrlShortenerError::StorageError)
                }
            }
            None => Err(UrlShortenerError::SlugNotFound),
        }
    }
}
