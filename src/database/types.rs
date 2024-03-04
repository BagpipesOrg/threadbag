use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Urldata {
    pub url: String,
}

// /saveUrl response output
#[derive(Debug, Deserialize, Serialize)]
pub struct UrlResponse {
    pub success: bool,
    pub shortUrl: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct job_start {
    pub scenario_id: String,
    pub delay: f32, // 32 should be enough
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUrlResponse {
    pub success: bool,
    pub longUrl: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericOut {
    pub success: bool,
    pub result: String,
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

#[derive(Serialize, Debug, Deserialize)]
pub struct ScenarioInfo {
    pub id: String,
}