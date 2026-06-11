use std::{sync::Arc, time::Duration};

use tokio::{sync::watch, time::sleep};

use crate::{
    JobResult,
    core::{broker::Broker, registry::Registry},
    worker::processor::process_job,
};

pub mod processor;

pub struct WorkerManager<B: Broker> {
    broker: B,
    registry: Arc<Registry>,
}

impl<B: Broker> WorkerManager<B> {
    pub fn new(broker: B, registry: Arc<Registry>) -> Self {
        Self { broker, registry }
    }

    pub async fn run(&self, mut shutdown_rx: watch::Receiver<bool>) -> JobResult<()> {
        println!("start running job loop......");

        loop {
            tokio::select! {
                changed = shutdown_rx.changed() => {
                    match changed {
                        Ok(()) => {
                            if *shutdown_rx.borrow() {
                                println!("received quit signal, exiting run loop...");
                                break;
                            }
                        },
                        Err(_) => {
                            break;
                        }
                    }
                }
                _ = self.run_job() => {}
            };
        }

        Ok(())
    }

    async fn run_job(&self) {
        let reg = self.registry.clone();

        match self.broker.deque().await {
            Ok(Some(payload)) => {
                // process job
                tokio::spawn(async move {
                    println!("[run_job] process job start...");
                    match process_job(&reg, payload).await {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("failed to run job: {}", e);
                        }
                    }
                });
            }
            Ok(None) => {
                // no job, wait 200ms
                sleep(Duration::from_millis(200)).await;
            }
            Err(e) => {
                eprintln!(" get job failed: {}", e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
