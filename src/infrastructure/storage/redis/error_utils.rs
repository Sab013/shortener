use crate::domain::errors::DomainErrors;
use redis::RedisResult;
use tracing::error;

pub trait RedisResultExt<T> {
    fn map_storage_err(self) -> Result<T, DomainErrors>;
}

impl<T> RedisResultExt<T> for RedisResult<T> {
    fn map_storage_err(self) -> Result<T, DomainErrors> {
        self.map_err(|err| {
            error!("Redis error: {}", err);
            DomainErrors::StorageError
        })
    }
}
