use redis::RedisError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobError {
    #[error("redis broker error: {0}")]
    RedisBrokerError(#[from] RedisError),

    #[error("decode job error: {0}")]
    DecodeJobError(String),

    #[error("job handler not found: {0}")]
    JobHandlerNotFound(String),
}
