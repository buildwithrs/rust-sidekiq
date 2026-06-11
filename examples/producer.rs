use std::time::Duration;

use rust_sidekiq::{
    JobResult,
    core::broker::{Broker, RedisBroker, new_redis_broker},
};
use tokio::{signal::ctrl_c, time::sleep};

#[tokio::main]
async fn main() -> JobResult<()> {
    let addr = "redis://127.0.0.1/?protocol=resp3";
    let queue = "My_Job_Queue";
    let broker = new_redis_broker(queue, addr)?;

    let mut idx = 0;

    loop {
        tokio::select! {
            _ = send_msg(&broker, idx) => {
                println!("sended msg: {idx}");
                if idx == 20 {
                    break;
                }

                idx += 1;
                sleep(Duration::from_millis(300)).await;

            }
            _ = ctrl_c() => {
                println!("user canceled");
                break;
            }
        }
    }

    Ok(())
}

async fn send_msg(broker: &RedisBroker, idx: usize) -> JobResult<()> {
    broker
        .enque("send-msg", &format!(r#"{{'msg': 'Hello - {idx}'}}"#))
        .await
}
