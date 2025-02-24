#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unreachable_code)]

use crate::core::error::Error;
use crate::database::types::GetUrlResponse;

use reqwest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::value::Value as SerdeValue;
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
    "https://api.bagpipes.io".to_string() //http://localhost:8080
}

// Define the struct representing the response
#[derive(Debug, Deserialize, Serialize)]
pub struct TxAssetTransferResponse {
    pub txdata: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenTxGen {
    chain: String,
    pallet_name: String,
    method_name: String,
    params: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenQueryGen {
    chain: String,
    pallet_name: String,
    method_name: String,
    params: String, //Vec<u8>,
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
    pub result: SerdeValue,
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

// download scenario data from api
pub async fn download_scenario_data(scenario_id: String) -> Result<String, Error> {
    let url = format!("{}/api/template/getUrl/{}", get_api_url(), scenario_id);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let response_data: GetUrlResponse = response.json().await?;
        //    println!("got response data: {:?}", response_data);
        let long_url = response_data.longUrl;
        //   println!("geturl longurl: {}", long_url);
        return Ok(long_url);
    } else {
        let error_message = response.text().await?;
        eprintln!("Error getting URL: {}", error_message);
        return Err(Error::NoEntryInDb);
    }

    Err(Error::NoEntryInDb)
}

// curl -X POST -H "Content-Type: application/json" -d '{"chain": "polkadot", "msg": "hack the planet"}' http://localhost:8080/api/actions/system-remark    -v
pub async fn system_remark(chain: String, msg: String) -> Result<generic_result, Error> {
    let client = Client::new();
    println!("System remark called with: {:?} {:?}", chain, msg);
    let request_body: RemarkRequest = RemarkRequest { chain, msg };

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
        assetin,
        assetout,
        amount,
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

pub async fn getwebhook_data(uuid: String) -> Result<SerdeValue, Error> {
    let url = format!("{}/api/webhook/fetchWebhookData/{}", get_api_url(), uuid);
    let client = Client::new();
    let resp = client
        .get(url)
        .timeout(Duration::from_secs(60))
        .send()
        .await?;

    let vl: SerdeValue = resp.json().await?;

    Ok(vl)
}

/*
curl -X POST -H "Content-Type: application/json" -d '{"chain": "polkadot", "pallet_name": "timestamp", "method_name": "now", "params": []}' http://localhost:8080/api/actions/query
*/
pub async fn query_chain(
    chain: String,
    pallet_name: String,
    method_name: String,
    params: String,
) -> Result<generic_result, Error> {
    let client = Client::new();

    let request_body: GenQueryGen = GenQueryGen {
        chain,
        pallet_name,
        method_name,
        params,
    };

    let url = format!("{}/api/actions/query", get_api_url());
    //   eprintln!("request_body: {:?}", request_body);
    //   eprintln!("making request: {:?}", url);
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;

    let body: generic_result = response.json().await?;
    //println!("Response body: {:?}", body);

    Ok(body)
}

/*
curl -X POST -H "Content-Type: application/json" -d '{"chain": "polkadot", "pallet_name": "System", "method_name": "remark", "params": ["0xDEADBEEF"]}' http://localhost:8080/api/actions/generic-tx-gen
{"result":"0x2004000010deadbeef"}

*/

pub async fn generic_tx_gen(
    chain: String,
    pallet_name: String,
    method_name: String,
    params: Vec<u8>, //Vec<u8>,
) -> Result<generic_result, Error> {
    let client = Client::new();

    let request_body: GenTxGen = GenTxGen {
        chain,
        pallet_name,
        method_name,
        params,
    };

    let url = format!("{}/api/actions/generic-tx-gen", get_api_url());

    println!("making request: {:?}", url);
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()
        .await?;

    let body: generic_result = response.json().await?;
    println!("Response body: {:?}", body);

    Ok(body)
}
