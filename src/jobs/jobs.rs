use crate::database::db::DBhandler;
use crate::database::decode::decompress_string;
use crate::error::Error;
use crate::jobs::types::Command;
use crate::scenarios::scenario_parse::verify_scenario_id;
use crate::scenarios::scenario_types::Graph;
use crate::scenarios::scenario_types::ScenarioSummary;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc; // use tokio's mpsc channel
use tokio::time::sleep;
use tokio::time::Duration;

use crate::scenarios::scenario_parse::multi_scenario_info;

/// Start a job worker for scenario_id and sleep for X(delay) amount of hours
pub async fn start_job_worker(scenario_id: String, delay: u64) -> Result<(), Error> {
    /// sanitize input | todo better verify function
    match verify_scenario_id(scenario_id.clone()) {
        true => {} // if its true do nothing and assume its a correct string0
        _ => return Err(Error::ScenarioIdNotFound),
    };

    println!("Starting job worker");
    let db_fluff = DBhandler::new();
    println!("Decoding data");
    db_fluff.save_log(scenario_id.clone(), "Starting worker".to_string())?;

    let graph: Graph = db_fluff
        .get_decoded_entry(scenario_id)
        .await
        .expect(return Err(Error::ScenarioIdNotFound));

    // parse the scenario to scenario summary list then
    let o2: Vec<ScenarioSummary> = multi_scenario_info(graph.clone());

    println!("scenario data extracted ");
    loop {
        sleep(Duration::from_secs(60 * 60 * delay)).await; // change me in the future, to work better with hours
    }
}

//change me
pub async fn dummy_thread(
    tx: tokio::sync::mpsc::Sender<Command>,
    ready: Arc<tokio::sync::Mutex<()>>,
) {
    // Wait for task_manager to be ready
    let _ = ready.lock().await;

    loop {
        // can_incr.increment();
        println!("Task manager is alive");

        //  println!("Db status: {:?}", status);
        if let Err(err) = tx
            .send(Command::Start {
                scenario_id: "scenarioid from test".to_string(),
                delay: 500u64,
            })
            .await
        {
            eprintln!("Failed to send command: {}", err);
        }

        sleep(Duration::from_secs(5)).await;
    }
}
