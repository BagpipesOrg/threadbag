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
        true => {
            println!("valida scenario id");
        } // if its true do nothing and assume its a correct string0
        _ => {
            println!("invalid scenario id: {:?}", scenario_id);
            return Err(Error::ScenarioIdNotFound);
        }
    };

    println!("Starting job worker");
    let db_fluff = DBhandler::new();
    println!("Decoding data");
    db_fluff.save_log(scenario_id.clone(), "Starting worker".to_string())?;
    db_fluff.save_log(scenario_id.clone(), "Decoding payload..".to_string())?;
    println!("decoding graph...");
    let graph: Graph = match db_fluff.get_decoded_entry(scenario_id.clone()).await {
        Ok(value) => value,
        _ => {
            println!("Error error");
            return Err(Error::ScenarioIdNotFound);
        }
    };
    println!("work thread was able to decode data to Graph");
    // parse the scenario to scenario summary list then
    let o2: Vec<ScenarioSummary> = multi_scenario_info(graph.clone());
    db_fluff.save_log(scenario_id.clone(), "Parsed scenario data".to_string())?;

    println!("scenario data extracted ");
    loop {
        let o3 = o2.clone();

        println!("----------Start---------------");
        for action_node in o3.iter() {
            println!("Displaying action data for scenario worker");
            println!("ScenarioSummary: {:?}", action_node);
            println!("Tx type: {:?}", action_node.txtype);
            println!("Dest chain: {:?}", action_node.dest_chain);
            println!("Source chain: {:?}", action_node.source_chain);
            let log_entry_go = format!(
                "Drafting {:?} tx from {} to {}",
                action_node.txtype, action_node.source_chain, action_node.dest_chain
            );
            db_fluff.save_log(scenario_id.clone(), log_entry_go.clone())?;
            println!("{:?}", log_entry_go);
        }

        println!("-------------EOL------------");
        db_fluff.save_log(scenario_id.clone(), "workload executed".to_string())?;
        db_fluff.save_log(scenario_id.clone(), "Sleeping worker.".to_string())?;
        println!("sleeping scenario worker");
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

        sleep(Duration::from_secs(5 * 10)).await;
    }
}
