use std::{collections::HashMap, pin::Pin};

use crate::JobError;
use crate::JobResult;

pub type JobHandler =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = JobResult<()>> + Send>> + Send + Sync>;

pub struct Registry {
    pub jobs: HashMap<String, JobHandler>,
}

impl Registry {
    pub fn new(cap: usize) -> Self {
        Self {
            jobs: HashMap::with_capacity(cap),
        }
    }

    pub fn add_job<F, Fut>(&mut self, name: &str, handler: F)
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = JobResult<()>> + Send + 'static,
    {
        let hdr = Box::new(move |payload| {
            Box::pin(handler(payload))
                as Pin<Box<dyn Future<Output = Result<(), JobError>> + Send + 'static>>
        });
        self.jobs.insert(name.to_string(), hdr);
    }

    pub fn get(&self, name: &str) -> Option<&JobHandler> {
        self.jobs.get(name)
    }
}
