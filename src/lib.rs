use crate::errors::JobError;

pub mod core;
pub mod errors;
pub mod worker;

pub type JobResult<T> = Result<T, JobError>;
