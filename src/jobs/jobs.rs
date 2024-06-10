use crate::chains::chains::get_token_decimals_by_chain_name;
use crate::database::db::{DBhandler, Loghandler};
//use crate::database::decode::decompress_string;
use crate::error::Error;
use crate::jobs::types::Command;
use crate::scenarios::pill_parse::process_chain_node;
use crate::scenarios::scenario_parse::convert_to_multinode;
//use crate::scenarios::scenario_parse::multi_scenario_info;
use crate::scenarios::scenario_parse::verify_scenario_id;
//use crate::scenarios::scenario_types::ScenarioSummary;
use crate::scenarios::pill_parse::process_node;
use crate::scenarios::scenario_types::{Graph, Graph2, MultiNodes};
use crate::scenarios::websockets::latest_webhookevents;
use crate::tx_format::lazy_gen::{
    generate_tx, generic_tx_gen, hydra_swaps, query_chain, system_remark,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
//use std::string;
use std::sync::Arc;
// use tokio's mpsc channel
use tokio::time::sleep;
use tokio::time::Duration;

fn hex_to_vec_u8(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("Invalid hex character"))
        .collect()
}

/// Start a job worker for scenario_id and sleep for X(delay) amount of hours
pub async fn start_job_worker(scenario_id: String, delay: u64) -> Result<(), Error> {
    // sanitize input | todo better verify function
    let log_db = Loghandler::new();
    match verify_scenario_id(scenario_id.clone()) {
        true => {
            println!("valid scenario id");
        } // if its true do nothing and assume its a correct string0
        _ => {
            println!("invalid scenario id: {:?}", scenario_id);
            log_db.insert_logs(scenario_id.clone(), "invalid scenario id".to_string())?;

            return Err(Error::ScenarioIdNotFound);
        }
    };

    println!("Starting job worker");
    let db_fluff = DBhandler::new();

    println!("Decoding data");
    log_db.insert_logs(scenario_id.clone(), "Starting worker".to_string())?;
    log_db.insert_logs(scenario_id.clone(), "Decoding payload..".to_string())?;
    println!("decoding graph...");
    let graph: Graph = match db_fluff.get_remote_entry(scenario_id.clone()).await {
        Ok(value) => value,
        Err(error) => {
            println!("Error error: {:?}", error);
            log_db.insert_logs(scenario_id.clone(), "Invalid scenario id".to_string())?;

            return Err(Error::ScenarioIdNotFound);
        }

        _ => {
            println!("got some type of error in the graph");
            log_db.insert_logs(
                scenario_id.clone(),
                "Could not parse the scenario data".to_string(),
            )?;

            return Err(Error::ScenarioIdNotFound);
        }
    };
    println!("Convert graph to graph2");
    log_db.insert_logs(
        scenario_id.clone(),
        "downloaded scenario data from api".to_string(),
    )?;

    let g2: Graph2 = convert_to_multinode(graph);
    println!("work thread was able to decode data to Graph");
    // parse the scenario to scenario summary list then
    //  let o2: Vec<ScenarioSummary> = multi_scenario_info(graph.clone());
    log_db.insert_logs(scenario_id.clone(), "Parsed scenario data".to_string())?;

    println!("scenario data extracted ");
    loop {
        let o3 = g2.clone();
        let mut webhook_loot: HashMap<String, JsonValue> = HashMap::new();

        println!("----------Start---------------");
        for action_node in o3.nodes {
            println!("looping at action node: {:?}", action_node);
            match action_node.clone() {
                MultiNodes::ChainTx(chainnode) => {
                    let mut chain_node = chainnode.clone();
                    let _ = process_chain_node(&mut chain_node, &webhook_loot);
                    log_db.insert_logs(scenario_id.clone(), "ChainTx Node detected".to_string())?;
                    let formdatan = chain_node.formData;
                    let mut local_chain: String = "test".to_string();
                    let mut method_name: String = "test".to_string();
                    let mut params: Vec<u8> = Vec::new();
                    let mut pallet_name: String = "test".to_string();

                    match formdatan {
                        Some(entry) => {
                            local_chain = entry.selectedChain.unwrap();
                            pallet_name = entry.selectedPallet.unwrap();
                            method_name = entry.selectedMethod.unwrap().name.unwrap();
                            for (key, value) in entry.params.unwrap().iter() {
                                let vec_u8 = hex_to_vec_u8(value);
                                params = vec_u8;
                                println!("Key: {}, Value: {}", key, value);
                            }
                        }
                        _ => return Err(Error::InvalidChainOption),
                    };
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "ChainTx Node has gathered information, will query api".to_string(),
                    )?;

                    let tx_gen =
                        generic_tx_gen(local_chain.clone(), pallet_name, method_name, params)
                            .await?;
                    log_db.insert_tx(
                        scenario_id.clone(),
                        '0'.to_string(),
                        local_chain,
                        "ChainTx".to_string(),
                        tx_gen.result.to_string(),
                    )?;
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "ChainTx Response was recieved".to_string(),
                    )?;

                    println!("chaintx detected!!!!!!!!!");
                }
                MultiNodes::ChainQuery(chainqnode) => {
                    println!("chainquery");
                    log_db
                        .insert_logs(scenario_id.clone(), "ChainQuery Node detected".to_string())?;

                    let mut chain_node = chainqnode.clone();
                    let _ = process_chain_node(&mut chain_node, &webhook_loot);
                    let formdatan = chain_node.formData;

                    let mut local_chain: String = "test".to_string();
                    let mut pallet_name: String = "test".to_string();
                    let mut method_name: String = "test".to_string();
                    let mut params: String = "test".to_string(); //params todo

                    match formdatan {
                        Some(entry) => {
                            local_chain = entry.selectedChain.unwrap();
                            pallet_name = entry.selectedPallet.unwrap();
                            method_name = entry.selectedMethod.unwrap().name.unwrap();
                            params = entry.methodInput.unwrap();
                        }
                        _ => return Err(Error::InvalidChainOption),
                    };
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "ChainQuery Node Request built".to_string(),
                    )?;
                    let tx_gen =
                        query_chain(local_chain.clone(), pallet_name, method_name, params).await?;
                    log_db.insert_tx(
                        scenario_id.clone(),
                        '0'.to_string(),
                        local_chain,
                        "ChainQuery".to_string(),
                        tx_gen.result.to_string(),
                    )?;
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "ChainQuery got response back".to_string(),
                    )?;

                    println!("chain query");
                }

                MultiNodes::Webhook(webhooknode) => {
                    println!("Webhook node!");

                    let uid = match webhooknode.formData {
                        Some(value) => value.uuid.unwrap(),
                        //.unwrap_or(return Err(Error::CouldNotFindWebhookData)),
                        _ => {
                            println!("some error");
                            return Err(Error::CouldNotFindWebhookData);
                        }
                    };
                    println!("got uiid");
                    let latest_data: HashMap<String, JsonValue> = latest_webhookevents(uid.clone())
                        .await
                        .unwrap_or(HashMap::new());
                    let upi = format!("Latest data got back from uuid: {:?}", latest_data);
                    println!("{}", upi);
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "Got webhook data from the api: ".to_string(),
                    )?;
                    log_db.insert_logs(scenario_id.clone(), upi)?;
                    webhook_loot.extend(latest_data);
                    log_db
                        .insert_logs(scenario_id.clone(), "Building webhook request".to_string())?;
                    let msg = format!("uuid for webhook is: {}", uid);
                    log_db.insert_logs(scenario_id.clone(), msg)?;

                    log_db.insert_logs(
                        scenario_id.clone(),
                        "Webhook finished, moving on ".to_string(),
                    )?;
                }
                MultiNodes::Action(chainnode) => {
                    let mut chain_node = chainnode.clone();
                    let _ = process_chain_node(&mut chain_node, &webhook_loot);
                    println!("action node detected");
                    let form_me = chain_node.clone().formData.expect("");
                    println!("form_me is; {:?}", form_me);
                    let txtype = form_me.action.expect("could not get tx type");

                    println!("Chain node: {:?}", chain_node);
                    println!("Formdata: {:?}", form_me.actionData);
                    //                  let log_entry_go =
                    //                     format!("Drafting {} tx from {} to {}", txtype, s_chain, d_chain);
                    println!("matching tx type: {:?}", txtype.clone());
                    match txtype.as_ref() {
                        "Remark" => {
                            let s_chain = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.chain")
                                .source
                                .chain;
                            println!("remark tx type detected");
                            let remarkme = match form_me
                                .actionData
                                .clone()
                                .expect("no source")
                                .source
                                .target
                            {
                                Some(value) => value,
                                _ => {
                                    println!("Could not find source target value, error");
                                    return Err(Error::ScenarioParseError);
                                }
                            };
                            println!("remark msg get: {:?}", remarkme);
                            // system_remark

                            println!("generating tx");
                            let remark_tx = match system_remark(s_chain.clone(), remarkme).await {
                                Ok(value) => value,
                                Err(error) => {
                                    println!("Error doing the system remark: {:?}", error);
                                    return Err(Error::ScenarioParseError);
                                }
                            }
                            .result;
                            log_db.insert_tx(
                                scenario_id.clone(),
                                0.to_string(),
                                s_chain,
                                "Remark".to_string(),
                                remark_tx.to_string().clone(),
                            )?;
                            println!("generated the following remark tx: {:?}", remark_tx);
                            println!("inserting logs");
                            log_db.insert_logs(
                                scenario_id.clone(),
                                "Remark transaction generated".to_string(),
                            )?;
                        }
                        "xTransfer" => {
                            println!("xTransfer tx type detected");
                            println!("action: {:?}", action_node);
                            let s_chain = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.chain")
                                .source
                                .chain;
                            let d_chain = form_me
                                .actionData
                                .clone()
                                .expect("could not get target.chain")
                                .target
                                .chain;
                            let _s_address = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.address")
                                .source
                                .address;
                            let d_address = form_me
                                .actionData
                                .clone()
                                .expect("could not get target address")
                                .target
                                .address
                                .expect("target address problem");

                            let d_amount = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.amount")
                                .source
                                .amount;
                            let s_assetid = form_me
                                .actionData
                                .clone()
                                .expect("could not get assetid")
                                .source
                                .assetId
                                .expect("no assetid")
                                .to_string();
                            // need to convert it to the raw balance using the asset decimals
                            let original_amount = d_amount
                                .parse::<u64>()
                                .expect("could not convert amount to u64");
                            let token_decimals = get_token_decimals_by_chain_name(&s_chain); // == one dot
                            let converted_amount =
                                original_amount * 10u64.pow(token_decimals as u32);
                            //             let s_chain = chain_node.source_chain.clone();
                            //            let d_chain = chain_node.dest_chain.clone();
                            //             let d_amount = chain_node.amount.clone();
                            //            let s_assetid = chain_node.assetid.clone();
                            //           let d_address = chain_node.dest_address.clone();
                            //           let tx_response: String =
                            println!("Converted amount: {:?}", converted_amount);
                            let tx_response = match generate_tx(
                                s_chain.clone(),
                                d_chain,
                                converted_amount.to_string(), // this should be
                                s_assetid,
                                d_address,
                            )
                            .await
                            {
                                Ok(value) => value.txdata, // if all good return the txdata
                                Err(error) => {
                                    println!("Error is: {:?}", error);
                                    "Could not generate transaction".to_string()
                                }
                                _ => "Could not generate transaction".to_string(),
                            };
                            println!("xTransfer tx: {:?}", tx_response);
                            log_db.insert_logs(scenario_id.clone(), tx_response.clone())?;
                            log_db.insert_tx(
                                scenario_id.clone(),
                                converted_amount.to_string(),
                                s_chain,
                                "xTransfer".to_string(),
                                tx_response,
                            )?;
                            log_db.insert_logs(
                                scenario_id.clone(),
                                "xTransfer transaction type".to_string(),
                            )?;
                        }
                        "swap" => {
                            //              hydra_swaps
                            let s_chain = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.chain")
                                .source
                                .chain;
                            let d_amount = form_me
                                .actionData
                                .clone()
                                .expect("could not get source.amount")
                                .source
                                .amount;
                            let d_assetid = form_me
                                .actionData
                                .clone()
                                .expect("could not get assetid")
                                .target
                                .assetId
                                .expect("no assetid");
                            println!("swap tx type detected");
                            let s_assetid = form_me
                                .actionData
                                .clone()
                                .expect("could not get assetid")
                                .source
                                .assetId
                                .expect("no assetid")
                                .to_string();
                            let tx_swap = match hydra_swaps(
                                s_assetid,
                                d_assetid.to_string(),
                                d_amount.clone(),
                            )
                            .await
                            {
                                Ok(swap_tx) => swap_tx.swap.swap_tx,
                                Err(error) => {
                                    println!("error: {:?}", error);
                                    log_db.insert_logs(
                                        scenario_id.clone(),
                                        "Could not generate swap transaction".to_string(),
                                    )?;
                                    return Err(Error::ScenarioParseError);
                                }
                            };
                            log_db.insert_tx(
                                scenario_id.clone(),
                                d_amount.to_string(),
                                s_chain,
                                "swap".to_string(),
                                tx_swap,
                            )?;
                            log_db.insert_logs(
                                scenario_id.clone(),
                                "Swap transaction generated".to_string(),
                            )?;

                            println!("swap tx okay");
                            log_db.insert_logs(
                                scenario_id.clone(),
                                "Swap transaction type".to_string(),
                            )?;
                        }

                        _ => {
                            println!("Unknown transaction type ");
                            log_db.insert_logs(
                                scenario_id.clone(),
                                "Unknown transaction type".to_string(),
                            )?;
                        }
                    };

                    //   println!("Log entry go: {:?}", log_entry_go);
                    //   log_db.insert_logs(scenario_id.clone(), log_entry_go.clone())?;

                    println!("Action node: {:?}", chain_node);
                    log_db
                        .insert_logs(scenario_id.clone(), "Building Action request".to_string())?;
                }
                MultiNodes::Chain(_chain_node) => {
                    //                  println!("Chain node: {:?}", chain_node);
                    log_db.insert_logs(
                        scenario_id.clone(),
                        "Building ChainNode request".to_string(),
                    )?;
                }
                MultiNodes::Http(chain_node) => {
                    log_db
                        .insert_logs(scenario_id.clone(), "scanning for pill nodes".to_string())?;
                    let mut node_copy = chain_node.clone();
                    let _ = process_node(&mut node_copy, &webhook_loot);
                    let url = node_copy.clone().formData.expect("").url;
                    let method = node_copy.clone().formData.expect("").method;
                    log_db.insert_logs(scenario_id.clone(), "pill node scan done".to_string())?;

                    println!("HTTP Node Method: {}", method);
                    println!("HTTP Node url: {}", url);
                    println!("Http node: {:?}", node_copy);
                    let req_fmt = format!("Making http request: Url: {} Method: {}", url, method);
                    log_db.insert_logs(scenario_id.clone(), req_fmt)?;
                    log_db.insert_logs(scenario_id.clone(), "Building http request".to_string())?;
                }
                MultiNodes::Unknown => {
                    println!("Unknown node");
                    log_db
                        .insert_logs(scenario_id.clone(), "Unknown request detected".to_string())?;
                }
                _ => {
                    println!("dont know");
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
