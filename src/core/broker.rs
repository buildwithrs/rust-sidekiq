use crate::{
    JobResult,
    core::job::{Job, encode_job},
    errors::JobError,
};
pub trait Broker: Send + Clone + Sync {
    fn enque(&self, name: &str, payload: &str) -> impl Future<Output = JobResult<()>> + Send;
    fn deque(&self) -> impl Future<Output = JobResult<Option<String>>> + Send;
}

pub fn new_redis_broker(queue: &str, addr: &str) -> Result<RedisBroker, JobError> {
    let conn = redis::Client::open(addr)?;
    Ok(RedisBroker::new(conn, queue))
}

#[derive(Debug, Clone)]
pub struct RedisBroker {
    conn: redis::Client,
    queue: String,
}

impl RedisBroker {
    pub fn new(conn: redis::Client, queue: &str) -> Self {
        Self {
            conn,
            queue: queue.to_string(),
        }
    }
}

impl Broker for RedisBroker {
    async fn enque(&self, name: &str, payload: &str) -> JobResult<()> {
        let mut conn = self.conn.get_multiplexed_async_connection().await?;

        let job = encode_job(&Job {
            name: name.to_string(),
            args: payload.to_string(),
        });

        let res = redis::cmd("LPUSH")
            .arg(&[self.queue.clone(), job])
            .exec_async(&mut conn)
            .await?;

        Ok(res)
    }

    async fn deque(&self) -> JobResult<Option<String>> {
        let mut conn = self.conn.get_multiplexed_async_connection().await?;

        let res = redis::cmd("RPOP")
            .arg(&self.queue.clone())
            .query_async(&mut conn)
            .await?;

        Ok(res)
    }
}
