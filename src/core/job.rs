use crate::{JobResult, errors::JobError};

#[derive(Debug)]
pub struct Job {
    pub name: String,
    pub args: String,
}

pub fn encode_job(job: &Job) -> String {
    format!("{}||{}", job.name, job.args)
}

pub fn decode_job(p: &str) -> JobResult<Job> {
    let parts: Vec<&str> = p.splitn(2, "||").collect();
    if parts.len() != 2 {
        return Err(JobError::DecodeJobError("bad job content".to_string()));
    }

    Ok(Job {
        name: parts[0].to_string(),
        args: parts[1].to_string(),
    })
}
