use crate::error::Error;
use reqwest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::Duration;

// api types
/*
"Content-Type: application/json"   -d '{
    "sourchain": "polkadot",
    "destchain": "assetHub",
    "assetid": "0",
    "amount": 1000000000,
    "destinationaddress": "5GYdCV9F3gg9gnmWU8nrt8tXCxMXDbcGpsdX1gJStCx9yZKK"
  }'
*/
//use std::collections::HashMap;

fn get_api_url() -> String {
    return "http://localhost:8080".to_string();
}

// Define the struct representing the response
#[derive(Debug, Deserialize, Serialize)]
pub struct TxAssetTransferResponse {
    pub txdata: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapResponse {
    pub success: bool,
    pub swap: swap_datan,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct swap_datan {
    pub swap_tx: String,
    pub scenarioid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct generic_result {
    pub result: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemarkRequest {
    chain: String,
    msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SwapRequest {
    pub assetin: String,
    pub assetout: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct api_xcm_request {
    source_chain: String,
    destchain: String,
    assetid: String,
    amount: String,
    destinationaddress: String,
}

/// queries api.bagpipes.io in order to generate the tx
/// generate_tx(source_chain: String, dest_chain: String, amount: String, assetid: String, dest_account: String)
pub async fn generate_tx(
    source_chain: String,
    dest_chain: String,
    amount: String,
    assetid: String,
    dest_account: String,
) -> Result<TxAssetTransferResponse, Error> {
    let client = Client::new();

    // Define the JSON payload
    let payload = json!({
        "sourchain": source_chain,//"polkadot",
        "destchain": dest_chain,//"assetHub",
        "assetid": assetid, //"0",
        "amount": amount,//1000000000,
        "destinationaddress": dest_account
    });
    println!("Built payload: {:?}", payload);
    // let api_base = get_api_url();//"https://api.bagpipes.io";
    let url = format!("{}/api/actions/xcm-asset-transfer", get_api_url());

    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await?;

    let body: TxAssetTransferResponse = response.json().await?;
    println!("Response body: {:?}", body);

    Ok(body)
}

// curl -X POST -H "Content-Type: application/json" -d '{"chain": "polkadot", "msg": "hack the planet"}' http://localhost:8080/api/actions/system-remark    -v
pub async fn system_remark(chain: String, msg: String) -> Result<generic_result, Error> {
    let client = Client::new();
    println!("System remark called with: {:?} {:?}", chain, msg);
    let request_body: RemarkRequest = RemarkRequest {
        chain: chain,
        msg: msg,
    };

    //   let api_base = get_api_url();//"https://api.bagpipes.io";
    let url = format!("{}/api/actions/system-remark", get_api_url());

    println!("making request: {:?}", url);
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;
    println!("Response: {:?}", response);
    let body: generic_result = response.json().await?;
    println!("Response body: {:?}", body);

    Ok(body)
}

/// HydraDX swaps
/// '{"assetin": "0", "assetout": "5", "amount": 100 }' http://localhost:8080/create/swap
/// (assetin: number, assetout: number, amount: number
pub async fn hydra_swaps(
    assetin: String,
    assetout: String,
    amount: String,
) -> Result<SwapResponse, Error> {
    let client = Client::new();
    println!("Input: {:?} {:?} {:?}", assetin, assetout, amount);
    let request_body: SwapRequest = SwapRequest {
        assetin: assetin,
        assetout: assetout,
        amount: amount,
    };

    // let api_base = ;//"https://api.bagpipes.io";
    let url = format!("{}/api/actions/swap/create", get_api_url());
    println!("url: {}", url);
    let response = client
        .post(url)
        .timeout(Duration::from_secs(60))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;
    //println!("response: {:?}", response.json());
    let body: SwapResponse = response.json().await?;
    println!("Response body: {:?}", body);

    Ok(body)
}
