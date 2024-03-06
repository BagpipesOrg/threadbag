#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct Graph {
    pub nodes: Vec<ChainNode>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct EdgeStyle {
    pub stroke: String,
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkerEnd {
    r#type: String,
    stroke_width: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelStyle {
    background_color: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChainNode {
    pub id: String,
    pub r#type: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeData {
    pub label: String,
    pub image: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub label: String,
    pub triggerToast: Option<bool>,
    pub image: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    pub chain: Option<String>,
    pub asset: Option<Asset>,
    pub address: Option<String>,
    pub amount: Option<String>,
    pub delay: Option<String>,   // Adjust the type as needed
    pub contact: Option<String>, // Adjust the type as needed
    pub action: Option<String>,
    pub actionData: Option<ActionData>,
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
    pub assetId: StringOrNumber,
    pub address: String,
    pub amount: String,
    pub symbol: Option<String>,
    pub delay: Option<String>, // Change the type accordingly
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub chain: String,
    pub assetid: StringOrNumber,
    pub address: Option<String>,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(f64),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub name: String,
    pub assetid: Option<u32>,
    pub symbol: Option<String>,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
