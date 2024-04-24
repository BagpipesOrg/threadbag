use reqwest;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use serde_json::json;

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

// Define the struct representing the response
#[derive(Debug, Deserialize, Serialize)]
pub struct TxAssetTransferResponse {
    pub txdata: String,
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
        "destinationaddress": "5GYdCV9F3gg9gnmWU8nrt8tXCxMXDbcGpsdX1gJStCx9yZKK"
    });
    let _api_base = "https://api.bagpipes.io";

    let response = client
        .post("https://api.bagpipes.io/xcm-asset-transfer")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await?;

    let body: TxAssetTransferResponse = response.json().await?;
    println!("Response body: {:?}", body);

    Ok(body)
}
