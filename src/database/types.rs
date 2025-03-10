#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::scenarios::scenario_types::ScenarioSummary;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Urldata {
    pub url: String,
}

// /saveUrl response output
#[derive(Debug, Deserialize, Serialize)]
pub struct UrlResponse {
    pub success: bool,
    pub shortUrl: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TxQueue {
    pub mempool: Vec<TxInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TxInfo {
    pub chain: String,
    pub amount: String,
    pub txType: String,
    pub Date: String,
    pub tx: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobStart {
    pub scenario_id: String,
    pub delay: u64, // f32 32 should be enough
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUrlResponse {
    pub success: bool,
    pub longUrl: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BroadcastInput {
    pub chain: String,
    pub tx: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BroadcastStatus {
    pub status: String,
    pub hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericOut {
    pub success: bool,
    pub result: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogsOut {
    pub success: bool,
    pub result: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScenarioInfoOut {
    pub success: bool,
    pub result: Option<Vec<ScenarioSummary>>,
}

#[derive(Debug, Deserialize)]
pub struct ScenarioRequest {
    source_chain: String,
    dest_chain: String,
    source_address: String,
    amount: u64,
    assetid: u64,
}

#[derive(Serialize)]
pub struct ScenarioResponse {
    success: bool,
    message: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct ScenarioLog {
    pub id: String,
    pub log_type: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ScenarioInfo {
    pub id: String,
}
