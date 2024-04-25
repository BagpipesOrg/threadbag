use crate::chains::chains::chain_info;
use crate::database::db::{DBhandler, Loghandler};
use crate::database::decode::decompress_string;
use crate::error::Error;
use crate::jobs::types::Command;
use crate::scenarios::scenario_parse::convert_to_multinode;
use crate::scenarios::scenario_parse::multi_scenario_info;
use crate::scenarios::scenario_parse::verify_scenario_id;
use crate::scenarios::scenario_types::ScenarioSummary;
use crate::scenarios::scenario_types::{Graph, Graph2, MultiNodes};
use crate::tx_format::lazy_gen::generate_tx;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc; // use tokio's mpsc channel
use tokio::time::sleep;
use tokio::time::Duration;

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
    let log_db = Loghandler::new();
    println!("Decoding data");
    log_db.insert_logs(scenario_id.clone(), "Starting worker".to_string())?;
    log_db.insert_logs(scenario_id.clone(), "Decoding payload..".to_string())?;
    println!("decoding graph...");
    let graph: Graph = match db_fluff.get_decoded_entry(scenario_id.clone()).await {
        Ok(value) => value,
        _ => {
            println!("Error error");
            return Err(Error::ScenarioIdNotFound);
        }
    };
    println!("Convert graph to graph2");

    let g2: Graph2 = convert_to_multinode(graph);
    println!("work thread was able to decode data to Graph");
    // parse the scenario to scenario summary list then
    //  let o2: Vec<ScenarioSummary> = multi_scenario_info(graph.clone());
    log_db.insert_logs(scenario_id.clone(), "Parsed scenario data".to_string())?;

    println!("scenario data extracted ");
    loop {
        let o3 = g2.clone();

        println!("----------Start---------------");
        for action_node in o3.nodes {
            match action_node {
                MultiNodes::Action(chain_node) => {
                    let form_me = chain_node.clone().formData.expect("");
                    let txtype = form_me.action.expect("could not get tx type");
                    let s_chain = form_me.actionData.clone().expect("could not get source.chain").source.chain;
                    let d_chain = form_me.actionData.clone().expect("could not get target.chain").target.chain;
                    let s_address = form_me.actionData.clone().expect("could not get source.address").source.address;
                    let d_address = form_me.actionData.clone().expect("could not get target address").target.address.expect("target address problem");

                    let d_amount = form_me.actionData.clone().expect("could not get source.amount").source.amount; 
                    let s_assetid = form_me.actionData.clone().expect("could not get assetid").source.assetId.expect("no assetid").to_string();
                              let log_entry_go = format!(
                                    "Drafting {} tx from {} to {}",
                                    txtype, s_chain, d_chain
                                );
                                println!("Log entry go: {:?}", log_entry_go);
                                log_db.insert_logs(scenario_id.clone(), log_entry_go.clone())?;

                    //             let s_chain = chain_node.source_chain.clone();
                    //            let d_chain = chain_node.dest_chain.clone();
                    //             let d_amount = chain_node.amount.clone();
                    //            let s_assetid = chain_node.assetid.clone();
                    //           let d_address = chain_node.dest_address.clone();
                    //           let tx_response: String =
                                let tx_response =   match generate_tx(s_chain, d_chain, d_amount, s_assetid, d_address).await {
                                      Ok(value) => value.txdata, // if all good return the txdata
                                     _ => "Could not generate transaction".to_string(),
                                };
                           log_db.insert_logs(scenario_id.clone(), tx_response.clone())?;

                    println!("Action node: {:?}", chain_node);
                    log_db
                        .insert_logs(scenario_id.clone(), "Building Action request".to_string())?;
                }
                MultiNodes::Chain(chain_node) => {
                    //                  println!("Chain node: {:?}", chain_node);
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "Building ChainNode request".to_string(),
                    )?;
                }
                MultiNodes::Webhook(chain_node) => {
                    println!("Webhook node: {:?}", chain_node);
                    log_db
                        .insert_logs(scenario_id.clone(), "Building webhook request".to_string())?;
                }
                MultiNodes::Http(chain_node) => {
                    let url = chain_node.clone().formData.expect("").url;
                    let method = chain_node.clone().formData.expect("").method;
                    println!("HTTP Node Method: {}", method);
                    println!("HTTP Node url: {}", url);
                    println!("Http node: {:?}", chain_node);
                    let req_fmt = format!("Making http request: Url: {} Method: {}", url, method);
                    log_db.insert_logs(scenario_id.clone(), req_fmt)?;
                    log_db.insert_logs(scenario_id.clone(), "Building http request".to_string())?;
                }
                MultiNodes::Unknown => {
                    println!("Unknown node");
                    log_db
                        .insert_logs(scenario_id.clone(), "Unknown request detected".to_string())?;
                }
            }
            /*
                println!("Displaying action data for scenario worker");
                println!("ScenarioSummary: {:?}", action_node);
                println!("Tx type: {:?}", action_node.txtype);
                println!("Dest chain: {:?}", action_node.dest_chain);
                println!("Source chain: {:?}", action_node.source_chain);
                let log_entry_go = format!(
                    "Drafting {:?} tx from {} to {}",
                    action_node.txtype, action_node.source_chain, action_node.dest_chain
                );
                log_db.insert_logs(scenario_id.clone(), log_entry_go.clone())?;

                let s_chain = action_node.source_chain.clone();
                let d_chain = action_node.dest_chain.clone();
                let d_amount = action_node.amount.clone();
                let s_assetid = action_node.assetid.clone();
                let d_address = action_node.dest_address.clone();
                let tx_response: String =
                    match generate_tx(s_chain, d_chain, d_amount, s_assetid, d_address).await {
                        Ok(value) => value.txdata, // if all good return the txdata
                        _ => "Could not generate transaction".to_string(),
                    };
                log_db.insert_logs(scenario_id.clone(), tx_response.clone())?;
                /*   */
                println!("tx_response is: {:?}", tx_response);
                println!("{:?}", log_entry_go);
            */
        }

        println!("-------------EOL------------");
        log_db.insert_logs(scenario_id.clone(), "workload executed".to_string())?;
        log_db.insert_logs(scenario_id.clone(), "Sleeping".to_string())?;

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
