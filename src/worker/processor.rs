use crate::{
    JobResult,
    core::{job::decode_job, registry::Registry},
    errors::JobError,
};

pub async fn process_job(reg: &Registry, payload: String) -> JobResult<()> {
    // 1. decode payload to -> name and job arg
    // 2. get handler from registry
    // 3. run the handler

    let job = decode_job(&payload)?;

    println!("[process_job] processing job: {:?}", job);

    let handler = reg.get(&job.name);
    if handler.is_none() {
        return Err(JobError::JobHandlerNotFound(job.name.to_string()));
    }

    let hdl = handler.unwrap();
    hdl(job.args).await
}
