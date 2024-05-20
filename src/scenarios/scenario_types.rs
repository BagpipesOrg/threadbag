#![allow(non_snake_case)]

use std::option;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Graph {
    pub nodes: Vec<ChainNode>, //Vec<ChainNode>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FutureGraph {
    pub nodes: Vec<MaNodes>, //Vec<ChainNode>,
    pub edges: Vec<Edge>,
}

/// Support Chain, HTTP and webhook node
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum MaNodes {
    chain(ChainNode),
    http(HTTPNode),
    action(ChainNode),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub style: EdgeStyle,
    pub marker_end: Option<MarkerEnd>,
    pub r#type: String,
    pub label: String,
    pub label_show_bg: Option<bool>,
    pub label_style: Option<LabelStyle>,
    pub focusable: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EdgeStyle {
    pub stroke: String,
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarkerEnd {
    r#type: String,
    stroke_width: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LabelStyle {
    pub background_color: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChainNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: Position,
    pub data: NodeData,
    pub style: serde_json::Value, // Adjust the type as needed
    pub formData: Option<FormData>,
    pub width: f64,
    pub height: f64,
    pub selected: bool,
    pub position_absolute: Option<Position>,
    pub dragging: bool,
}

/// for parsing http nodes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HTTPNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: Position,
    pub data: NodeData,
    pub style: serde_json::Value, // Adjust the type as needed
    pub formData: Option<HTTP_NODE_FORMDATA>,
    pub width: f64,
    pub height: f64,
    pub selected: bool,
    pub position_absolute: Option<Position>,
    pub dragging: bool,
}

/// for parsing webhooks
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebhookNode {
    pub id: String,
    pub r#type: String,
    pub position: Position,
    pub data: NodeData,
    pub style: serde_json::Value, // Adjust the type as needed
    pub formData: Option<WebhookFormData>,
    pub width: f64,
    pub height: f64,
    pub selected: bool,
    pub position_absolute: Option<Position>,
    pub dragging: bool,
    pub selectedWebhook: String,
    pub uuid: String,
    pub eventData: Option<WebhookEventData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeData {
    pub label: String,
    pub image: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub label: String,
    pub triggerToast: Option<bool>,
    pub image: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HTTP_NODE_FORMDATA {
    pub serializeUrl: String,
    pub parseResponse: String,
    pub shareCookies: String,
    pub rejectUnverifiedCertificates: String,
    pub followRedirects: String,
    pub followAllRedirects: String,
    pub requestCompressedContent: String,
    pub useMutualTLS: String,
    pub evaluateErrors: String,
    pub url: String,
    pub method: String,
    pub connectionType: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebhookFormData {
    pub uuid: String,
    pub webhookName: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebhookEventData {
    pub query: Option<String>,
    pub createdAt: String,
    pub method: String,
    // add more after needs
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph2 {
    pub nodes: Vec<MultiNodes>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MultiNodes {
    #[serde(rename = "http")]
    Http(HTTPNode),
    #[serde(rename = "chain")]
    Chain(ChainNode),
    #[serde(rename = "action")]
    Action(ChainNode),
    #[serde(rename = "webhook")]
    Webhook(ChainNode),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormData {
    pub chain: Option<String>,
    pub asset: Option<Asset>,
    pub address: Option<String>,
    pub amount: Option<String>,
    pub delay: Option<String>,   // Adjust the type as needed
    pub contact: Option<String>, // Adjust the type as needed
    pub action: Option<String>,
    pub actionData: Option<ActionData>,
    pub serializeUrl: Option<String>,
    pub parseResponse: Option<String>,
    pub shareCookies: Option<String>,
    pub rejectUnverifiedCertificates: Option<String>,
    pub followRedirects: Option<String>,
    pub followAllRedirects: Option<String>,
    pub requestCompressedContent: Option<String>,
    pub useMutualTLS: Option<String>,
    pub evaluateErrors: Option<String>,
    pub url: Option<String>,
    pub method: Option<String>,
    pub connectionType: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionData {
    pub actionType: String,
    pub source: Source,
    pub target: Target,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub chain: String,
    pub assetId: Option<StringOrNumber>,
    pub address: String,
    pub amount: String,
    pub symbol: Option<String>,
    pub delay: Option<String>,  // Change the type accordingly
    pub target: Option<String>, // used for remark
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub chain: String,
    pub assetId: Option<StringOrNumber>,
    pub address: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}

impl StringOrNumber {
    pub fn to_string(&self) -> String {
        match self {
            StringOrNumber::String(s) => s.clone(), // Extract and return the inner String
            StringOrNumber::Number(n) => n.to_string(), // Convert f64 to String
        }
    }
}

// impl into()
impl From<StringOrNumber> for String {
    fn from(value: StringOrNumber) -> Self {
        match value {
            StringOrNumber::String(s) => s,
            StringOrNumber::Number(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asset {
    pub name: String,
    pub assetid: Option<u32>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TxType {
    xTransfer,
    swap,
    unknown,
}

impl From<String> for TxType {
    fn from(value: String) -> TxType {
        let swap: String = "swap".to_string();
        let xtransfer: String = "xTransfer".to_string();

        match value {
            xtransfer => TxType::xTransfer,
            swap => TxType::swap,
            _ => TxType::unknown,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct scenario_summary {
    pub source_chain: String,
    pub source_address: String, // senders address
    pub dest_chain: String,     // destination chain
    pub dest_address: String,   // destination address
    pub assetid: Option<String>,
    pub amount: Option<String>, // transfer amount
    pub txtype: String,         // would be enum in rust..
    pub tx: String,             // hex tx
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScenarioSummary {
    pub source_chain: String,
    pub source_address: String,
    pub dest_chain: String,
    pub dest_address: String,
    pub assetid: String, // This can be a string or a number in Rust
    pub amount: String,  // This can be a string or a number in Rust
    pub txtype: TxType,  // Enum representing txtype
    pub tx: String,
}
