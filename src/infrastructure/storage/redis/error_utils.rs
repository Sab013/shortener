use redis::RedisResult;
use crate::domain::errors::DomainErrors;

pub trait RedisResultExt<T> {
    fn map_storage_err(self) -> Result<T, DomainErrors>;
}

impl<T> RedisResultExt<T> for RedisResult<T> {
    fn map_storage_err(self) -> Result<T, DomainErrors> {
        self.map_err(|_| DomainErrors::StorageError)
    }
}