use std::sync::Arc;

use rust_sidekiq::{
    JobResult,
    core::{
        broker::{Broker, new_redis_broker},
        registry::Registry,
    },
    worker::WorkerManager,
};
use tokio::{signal::ctrl_c, sync::watch};

#[tokio::main]
async fn main() -> JobResult<()> {
    let addr = "redis://127.0.0.1/?protocol=resp3";
    let queue = "My_Job_Queue";
    let broker = new_redis_broker(queue, addr)?;

    broker.enque("send-msg", r#"{'msg': 'Hello'}"#).await?;
    broker
        .enque("send-msg", r#"{'msg': 'Hello Again'}"#)
        .await?;

    let mut registry = Registry::new(10);
    registry.add_job("send-msg", send_msg);
    registry.add_job("send-email", send_email);

    let worker = WorkerManager::new(broker, Arc::new(registry));

    let (shutdown_tx, shutdown_rx) = watch::channel::<bool>(false);

    tokio::select! {
        _ = worker.run(shutdown_rx) => {}
        _ = ctrl_c() => {
            println!("user canceled");
            match shutdown_tx.send(true) {
                Err(e) => {
                    eprintln!("send true to shutdown_tx: {}", e);
                }
                Ok(_) => {}
            }
        }
    }

    Ok(())
}

async fn send_msg(args: String) -> JobResult<()> {
    println!("sending msg: {}", args);
    Ok(())
}

async fn send_email(args: String) -> JobResult<()> {
    println!("sending email: {}", args);
    Ok(())
}
