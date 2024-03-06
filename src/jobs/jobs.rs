use crate::jobs::types::Command;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc; // use tokio's mpsc channel
use tokio::time::sleep;
use tokio::time::Duration;

/// Start a job worker for scenario_id and sleep for X(delay) amount of hours
pub async fn start_job_worker(scenario_id: String, delay: u64) {
    loop {
        sleep(Duration::from_secs(60 * 60 * delay)).await; // change me in the future, to work better with hours
    }
}

pub async fn dummy_thread(
    tx: tokio::sync::mpsc::Sender<Command>,
    ready: Arc<tokio::sync::Mutex<()>>,
) {
    // Wait for task_manager to be ready
    ready.lock().await;

    loop {
        // can_incr.increment();
        println!("Task manager is alive");

        if let Err(err) = tx
            .send(Command::Start {
                job: "scenarioid from test".to_string(),
            })
            .await
        {
            eprintln!("Failed to send command: {}", err);
        }

        sleep(Duration::from_secs(5)).await;
    }
}
