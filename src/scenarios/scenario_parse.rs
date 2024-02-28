use getrandom::getrandom;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChainNode {
    id: String,
    r#type: String,
    position: Position,
    data: NodeData,
    style: serde_json::Value, // Adjust the type as needed
    formData: FormData,
    width: f64,
    height: f64,
    selected: bool,
    position_absolute: Position,
    dragging: bool,
}

#[derive(Debug, Deserialize)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
struct NodeData {
    label: String,
    image: String,
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    label: String,
    triggerToast: Option<bool>,
    image: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FormData {
    chain: String,
    asset: Asset,
    address: String,
    amount: String,
    delay: Option<serde_json::Value>,   // Adjust the type as needed
    contact: Option<serde_json::Value>, // Adjust the type as needed
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    asset_id: u32,
    symbol: Option<String>,
}

#[derive(Debug)]
enum TxType {
    xTransfer,
    swap,
}

#[derive(Debug, Deserialize)]
struct scenario_summary {
    source_chain: String,
    source_address: String, // senders address
    dest_chain: String,     // destination chain
    dest_address: String,   // destination address
    assetid: Option<String>,
    amount: Option<String>, // transfer amount
    txtype: String,         // would be enum in rust..
    tx: String,             // hex tx
}

#[derive(Debug)]
struct ScenarioSummary {
    source_chain: String,
    source_address: String,
    dest_chain: String,
    dest_address: String,
    asset_id: String, // This can be a string or a number in Rust
    amount: String,   // This can be a string or a number in Rust
    txtype: TxType,   // Enum representing txtype
    tx: String,
}

pub fn generate_random_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$*+_"; // supported charset
    const ID_LENGTH: usize = 9; // size / length
    let mut random_bytes = vec![0; ID_LENGTH];
    getrandom(&mut random_bytes).expect("Failed to generate random bytes");

    // Convert random bytes to a string using the charset
    let random_id: String = random_bytes
        .iter()
        .map(|byte| CHARSET[*byte as usize % CHARSET.len()] as char)
        .collect();
    random_id
}

// Implement a function for validation
fn validate_diagram_data(
    diagram_data: &serde_json::Value,
) -> Result<&serde_json::Value, Box<dyn std::error::Error>> {
    // DiagramData
    println!("Inside validateDiagramData");

    // Access nodes and edges
    let nodes = diagram_data["nodes"].as_array().ok_or("Nodes not found")?;
    // let edges = diagram_data["edges"].as_array().ok_or("Edges not found")?;

    // Check if there are multiple starting nodes
    let starting_nodes: Vec<_> = nodes
        .iter()
        .filter(|node| {
            !diagram_data["edges"]
                .as_array()
                .unwrap()
                .iter()
                .any(|edge| edge["target"] == node["id"])
        })
        .collect();

    if starting_nodes.len() > 1 {
        return Err("There are multiple starting nodes. Please make sure there's only one starting point in the diagram.".into());
    }

    // Check for multiple ending nodes
    let ending_nodes: Vec<_> = nodes
        .iter()
        .filter(|node| {
            !diagram_data["edges"]
                .as_array()
                .unwrap()
                .iter()
                .any(|edge| edge["source"] == node["id"])
        })
        .collect();

    if ending_nodes.len() > 1 {
        return Err("There are multiple ending nodes. Please make sure there's only one ending point in the diagram.".into());
    }

    // TODO: Check for circular references (This will need a more advanced algorithm)

    // TODO: Check for multiple paths (Another advanced algorithm)

    // Ensure action nodes are not at the start or end
    if let (Some(starting_node), Some(ending_node)) = (starting_nodes.get(0), ending_nodes.get(0)) {
        if starting_node["type"] == "action" || ending_node["type"] == "action" {
            return Err("Scenarios cannot start or end with an action node.".into());
        }
    }

    // Ensure that chain nodes or action nodes are not connected directly
    for edge in diagram_data["edges"].as_array().unwrap() {
        let source_type = nodes
            .iter()
            .find(|node| node["id"] == edge["source"])
            .map(|node| node["type"].as_str().unwrap_or_default());

        let target_type = nodes
            .iter()
            .find(|node| node["id"] == edge["target"])
            .map(|node| node["type"].as_str().unwrap_or_default());

        if let (Some(source_type), Some(target_type)) = (source_type, target_type) {
            if (source_type == "chain" && target_type == "chain")
                || (source_type == "action" && target_type == "action")
            {
                return Err("Chain nodes or action nodes are connected to each other directly. They should be connected as ChainNode > ActionNode > ChainNode.".into());
            }
        }
    }

    // Additional validation checks can go here

    // Return Ok with the original diagramData if no issues found
    Ok(diagram_data)
}
