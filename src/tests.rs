#[cfg(test)]
mod tests {

    use crate::scenarios::scenario_parse::convert_to_multinode;
    // use crate::scenarios::scenario_types::ScenarioSummary;
    //use crate::scenarios::scenario_types::TxType;

    use crate::scenarios::pill_parse::{process_chain_node, process_node};

    use crate::scenarios::scenario_types::{Graph2, MultiNodes};
    use crate::scenarios::websockets::latest_webhookevents;
    use crate::{database::db::DBhandler, tx_format::lazy_gen::query_chain};

    use crate::web_server::http::{quick_server, run_webserver, spawn_web_server};
    use reqwest;
    use reqwest::Client;
    use serde_json::Value;
    use std::collections::HashMap;
    // use actix_rt::System;
    //    use std::fmt::format;
    //use crate::chains::chains::{chains, get_rpc_endpoint};
    //use crate::database::db::DBhandler;
    //use crate::database::decode::decompress_string;
    //  use crate::scenarios::scenario_types::Graph;
    //    use crate::tx_format::generic::{
    //        assethubpolkadotconf, hydradxconf, interlayconf, polkadotconf,
    //   };
    //    use subxt::{OnlineClient, PolkadotConfig};
    //   use subxt_signer::sr25519::dev;

  //  #[actix_web::test]
    async fn it_works() {
        println!("running");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }



  //   #[actix_web::test]
    async fn it_work2() {
        println!("running");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

   // #[actix_web::test]
    async fn test_webserver() -> Result<(), anyhow::Error> {
        //    let _system = System::new();
        println!("starting server");
        //quick_server
        let (Server, Port) = quick_server().expect("Couldnt start http server");
        println!("server spawned on port: {Port}");
        let _ = tokio::spawn(Server);
        println!("testing urls");

        let url = format!("http://localhost:{}/", Port);

        let response = reqwest::get(&url).await?;

        assert_eq!(response.status().is_success(), true);
        println!("test passed");
        println!("test web server");
        Ok(())
    }

    #[actix_web::test]
    async fn test_websocks() -> Result<(), anyhow::Error> {
        let multi_scenario_id: String = "LSm-41cJY".to_string();
        let db_h = DBhandler::new();
        println!("donwloading scenario data");
        let out = db_h.get_remote_entry(multi_scenario_id).await.unwrap();
        println!("donwloaded scenario data");

        let g2: Graph2 = convert_to_multinode(out);
        let ulti_list = g2.nodes;
        let mut webhook_loot: HashMap<String, Value> = HashMap::new();
        for node in ulti_list {
            match node {
                MultiNodes::Webhook(node) => {
                    //      println!("webhook node detected: {:?}", node);
                    let uuid = node.formData.unwrap().uuid.unwrap();
                    println!("got the uuid: {}", uuid);
                    let latest_data: HashMap<String, Value> =
                        latest_webhookevents(uuid).await.unwrap();
                    println!("Latest data got back from uuid: {:?}", latest_data);
                    webhook_loot.extend(latest_data);
                }
                MultiNodes::Http(node) => {
                    println!("http node detected: {:?}", node);
                    let mut node_copy = node.clone();
                    let _ = process_node(&mut node_copy, &webhook_loot);
                    println!("New node: {:?}", node_copy);
                    // let url_test_data = node.formData.unwrap().url;
                    //  println!("Extracted url data: {:?}", url_test_data);
                }

                MultiNodes::ChainQuery(chainnode) => {
                    println!("ChainQuery node");
                    println!("pre query node: {:?}", chainnode);
                    let mut node_copy = chainnode.clone();
                    let _ = process_chain_node(&mut node_copy, &webhook_loot);
                    println!("Proccessed chain query node: {:?}", node_copy);
                    let fm = node_copy.formData.unwrap();
                    let local_chain = fm.selectedChain.unwrap();
                    let pallet_name = fm.selectedPallet.unwrap();
                    let method_name = fm.selectedMethod.unwrap().name.unwrap();
                    let inputen = fm.methodInput.unwrap();
                    let tx_gen = query_chain(local_chain, pallet_name, method_name, inputen)
                        .await
                        .unwrap();
                    println!("Txgen: {:?}", tx_gen);
                    //  let output = query_chain(chain, ).await;
                }

                _ => {
                    println!("other node");
                }
            }
            // println!("Node is: {:?}", node);
        }
        println!("websockets");
        let _uuid: String = "885d929c-2016-46ed-bb11-9ee59f784b12".to_string();
        //      let latest_data = latest_webhookevents(uuid).await.unwrap();
        //       println!("Latest data got back: {:?}", latest_data);
        println!("Webhook loot data: {:?}", webhook_loot);
        println!("end of websocket tests");
        Ok(())
    }

    #[actix_web::test]
    async fn test_mulit_type_node() -> Result<(), anyhow::Error> {
        /*
                println!("Starting test_mulit_type_node");
                let db_h = DBhandler::new();
                //   let db = db_h.read_db()?;

                let multi_scenario_id: String = "PLojHUPSl".to_string();

                let out = db_h.get_entry(multi_scenario_id).unwrap();
                let decoded = decompress_string(out)
                    .await
                    .expect("Failed to decompress string, invalid value");
                println!("decoded ok");
             //   println!("Decoded as: {}", decoded);
                // Decoded diagram data json
                let graph_result: SerdeResult<Graph> = serde_json::from_str(decoded.as_str());

                let g2: Graph2 = convert_to_multinode(graph_result.expect("could not parse graph"));
                let ulti_list = g2.nodes;

                println!("Amount in loot: {}", ulti_list.len());
                let http_node_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::Http(_)))
                    .count();
                let chain_q_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::ChainQuery(_)))
                    .count();
                let chain_tx_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::ChainTx(_)))
                    .count();
                let chain_node_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::Chain(_)))
                    .count();
                let action_node_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::Action(_)))
                    .count();
                let webhook_node_count = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::Webhook(_)))
                    .count();
                let unknown = ulti_list
                    .iter()
                    .filter(|node| matches!(node, MultiNodes::Unknown))
                    .count();

                println!("http_node_count: {}", http_node_count);
                println!("chain_node_count: {}", chain_node_count);
                println!("action_node_count: {}", action_node_count);
                println!("webhook_node_count: {}", webhook_node_count);
                println!("Chaintx: {}", chain_tx_count);
                println!("unknown: {}", unknown);

                println!("chain_q_count: {}", chain_q_count);

                println!("decoded okay");
                // parse scenario
                println!("parsing scenario_information");

                //   let output_string =
                //     scenario_information(graph.clone()).expect("could not parse scenario");
        */
        return Ok(());
    }

    /*
        #[actix_rt::test]
        async fn make_api_request() -> Result<(), anyhow::Error> {
            println!("make_api_request start");
            let sourcechain = "polkadot".to_string();
            let destchain = "assetHub".to_string();
            let assetid = "0".to_string();
            let amount = "100000000".to_string();
            let destinationaddress = "5GYdCV9F3gg9gnmWU8nrt8tXCxMXDbcGpsdX1gJStCx9yZKK".to_string();
            //  dest_chain: String, amount: String, assetid: String, dest_account: String)
            let tx_response = generate_tx(sourcechain, destchain, amount, assetid, destinationaddress)
                .await
                .unwrap(); // yolo
            println!("Got tx back: {:?}", tx_response.txdata);
            println!("make_api_request finished");
            Ok(())
        }
    */
    /*

        #[actix_rt::test]
        async fn read_db_entries() {
            let mut db_h = DBhandler {};
            let db = db_h.read_db().unwrap();
            let iterme = db.iter();
            println!("Printing values in database");
            for kv in iterme {
                match kv {
                    Ok((key, value)) => {
                        let str_key = String::from_utf8(key.to_vec()).unwrap();
                        println!("String key: {:?}", str_key);
                        let str_val = String::from_utf8(value.to_vec()).unwrap();
                        println!("String value: {:?}", str_val);
                        //let decode_me =

                        /*
                        match decompress_string(str_val).await {
                            Ok(value) => {
                                println!("managed to decode entry");
                            }
                            Err(err) => {
                                println!("could not decode entry, Error: {:?}", err);
                            }
                        }
                                */

                        println!("entry return okay!");
                    }
                    Err(err) => {
                        println!("error reading entry");
                    }
                }
            }
            println!("--.---Reading db entries done---.--");
            println!("test start");
            // valid encoded scenario data
            let inp = "eJzdV9tum0AQ/ZVon1rJpuyFXfBbajdJlVa9xG1TVVG0wBojY0gXiONG/vfOGvCF3JomUdW8wczOzOHMnFn7EqVZqHLU+3GJ4hD1UDCWcXrKfcfBLuqgYn6mGiu8nmV5XMRZinqX6AL1sLBswRnHxBOYYpd20Bz1uszlFmWe63BGhWCMLToolIU0UYn0VQIZ+3XGeCojU8F6taxh5ecRWFM5VatDEJ0X8wQMl/A4yvR0UCerYPUAVjKRYVZApMxzVRhfnaJxDdbet/CdNuScT/3MQBl8GJoaMgy1yoEK5Bx8/zw//+Qdn++XXkr3DwY/Z+/8w6+6nB+R6aT/c/j+eO9w4oCnfxxHeybxNCtTqIswvIQqkcBDWiZJBwVZWsigqF6hzCwOizHqEcY6aKziaAwuxjDgUYkKCgXgRjLJ1ZrsXT/PkrJQf0G6llEUp1GdctHZarJvhzwQdzZZYNdyCOeCYEEZd0hdz+OWR7kniCAe8Wzy1E0ez+FzBhfX9Phjq/2mwchB/2+L70P5DS0GPJD31Kcj7m4IuTJfo2SMPYs6nufaDmVQANclwSyozYSNbWG7zL2my3XOmztYH+ihfCbP0AZBNl4TRAjZJKjQ5S0SuA/aWzVA3ECF7O5FxxxuMepASwX2PC6ahgAMhgm2XY85AOXfaeDL0WC486JRws6uce8clP7LLVFge1MVJubRZFFJ4YlX3z3acLsuXEEDMrpbF9QllkMF5tzlMGkGualIoSKzGSWAhdqOjZ0H6eJiqGWaj5Q2fC6N20eGFcjNY3lW6kDddgnW19zj7bz2Ni2kjqoZvGE8V3v4QRi2yy4214e9sT44+fM5uldXW4N00kEqjK7+ZKpu02577zZ9unLn1uRdWdTNrMCDziYmMtJKpUuPMXyrPx6gTaWeKP0mDc3xZpC1zmbtw3ixGvQ8np4lyle/4uUYNbPaPB6Ns9nrqFm/lakB5MtgEmkYiLCfJZleIVtOdVDm0jfnTOT1t1C3tXJX1Fy5qxpu2gHPkJpqEXVb09GmZr2utqlZBzwnajb/g3TbDLQEtf6n0hLUOuA5UHOy+A1iJVCZ".to_string();
            //   let funkar = decompress_string(inp).await.unwrap(); //will fail if not decode is ok
            //   let comp: String = compress_string("test").await.unwrap();
            // println!("Comp: {:?}", comp);
            let dec: String = decompress_string(inp).await.unwrap();
            println!("Decoded diagram data json as a string");
            let graph: Graph = serde_json::from_str(dec.as_str()).expect("Failed to parse JSON");
            println!("Decoded diagram data to a rust object");
            // verify that it can decompress the compressed data correctly
            //  assert_eq!("test".to_string(), "test");
            println!("decoded: {:?}", dec);

            println!("test done, string decompressed ok");
        }


    #[actix_rt::test]
    async fn polkadot_2_assethub_tx() -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting..");
        let api =
            OnlineClient::<PolkadotConfig>::from_url(get_rpc_endpoint(chains::Polkadot)).await?;
        println!("polkadot_2_assethub_tx test");
        let dest = dev::bob().public_key().into();
        let tx = polkadotconf::tx()
            .balances()
            .transfer_allow_death(dest, 10_000);
        //      let encoded_tx = tx.encode();

        // Convert the encoded transaction into a hexadecimal string
        //let hex_tx = hex::encode(encoded_tx);
        let unsigned_extrinsic = api.tx().create_unsigned(&tx)?;
        let hex_tx = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));

        //let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));
        //  println!("unsigned extrinsic: {:?}", unsigned_extrinsic.encoded());

        println!("tx is: {:?}", hex_tx);
        Ok(())
    }
      */
}
