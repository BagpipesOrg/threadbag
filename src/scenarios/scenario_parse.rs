use crate::error::Error;
use crate::scenarios::scenario_types::{Graph, Graph2, MultiNodes, ScenarioSummary, TxType};
use crate::scenarios::scenario_types::{HTTPNode, HTTP_NODE_FORMDATA};
use getrandom::getrandom;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$*+_"; // supported charset
const ID_LENGTH: usize = 9; // size / length

// scenario_info
pub fn scenario_information(inputen: Graph) -> Result<String, Error> {
    let mut chain_list: Vec<String> = Vec::new();

    for node in inputen.nodes {
        if node.node_type == "chain" {
            if let Some(formData) = node.formData {
                chain_list.push(formData.chain.unwrap());
            }
        } else if node.node_type == "action" && node.formData.is_some() {
            if let Some(action) = node.formData.unwrap().action {
                chain_list.push(action);
            }
        }
    }
    let formatted_chain_list = chain_list.join(" > ");
    Ok(formatted_chain_list)
}

// Return a parsed list of different types of nodes
pub fn convert_to_multinode(scenario_data: Graph) -> Graph2 {
    let mut ulti_list: Vec<MultiNodes> = Vec::new();

    // println!("got graph back");

    for node in scenario_data.nodes {
        //          println!("Node is {:?}", node.node_type);
        match node.node_type.as_str() {
            "chain" => {
                ulti_list.push(MultiNodes::Chain(node));
            }
            "action" => {
                ulti_list.push(MultiNodes::Action(node));
            }
            "webhook" => {
                ulti_list.push(MultiNodes::Webhook(node));
            }
            "chainTx" => {
                ulti_list.push(MultiNodes::ChainTx(node));
            }
            "chainQuery" => {
                ulti_list.push(MultiNodes::ChainQuery(node));
            }
            "http" => {
                let formdata = node.formData.expect("could not find formdata");
                //     println!("http Formdata: {:?}", formdata);
                let http_node: HTTPNode = HTTPNode {
                    id: node.id,
                    node_type: "http".to_string(),
                    position: node.position,
                    data: node.data,
                    style: node.style,
                    formData: Some(HTTP_NODE_FORMDATA {
                        serializeUrl: formdata.serializeUrl.expect("could not get formdata"),
                        parseResponse: formdata.parseResponse.expect(""),
                        shareCookies: formdata.shareCookies.expect(""),
                        rejectUnverifiedCertificates: formdata
                            .rejectUnverifiedCertificates
                            .expect(""),
                        followAllRedirects: formdata.followAllRedirects.expect(""),
                        followRedirects: formdata.followRedirects.expect(""),
                        requestCompressedContent: formdata.requestCompressedContent.expect(""),
                        useMutualTLS: formdata.useMutualTLS.expect(""),
                        evaluateErrors: formdata.evaluateErrors.expect(""),
                        url: formdata.url.expect(""),
                        method: formdata.method.expect(""),
                        connectionType: formdata.connectionType.expect(""),
                    }),
                    width: node.width,
                    height: node.height,
                    selected: node.selected,
                    position_absolute: node.position_absolute,
                    dragging: node.dragging,
                };
                ulti_list.push(MultiNodes::Http(http_node));
            }

            _ => {}
        }
    }

    println!("Amount in loot: {}", ulti_list.len());
    Graph2 {
        nodes: ulti_list,
        edges: scenario_data.edges,
    }
}

pub fn multi_scenario_info(scenario_data: Graph) -> Vec<ScenarioSummary> {
    let mut alles: Vec<ScenarioSummary> = Vec::new();

    for node in &scenario_data.nodes {
        if node.node_type == "action"
            && node.formData.is_some()
            && node.formData.as_ref().unwrap().actionData.is_some()
        {
            let action_data = node.formData.as_ref().unwrap().actionData.as_ref().unwrap();

            let mut tmp_scenario = ScenarioSummary {
                source_chain: "not set".to_string(),
                source_address: "not set".to_string(),
                dest_chain: "not set".to_string(),
                dest_address: "not set".to_string(),
                assetid: "0".to_string(), // assuming assetId is a number
                amount: "0".to_string(),
                txtype: TxType::unknown,
                tx: "not set".to_string(),
            };
            //     println!("action data: {:?}", action_data);
            let _ss = "swap".to_string();
            let _xt = "xTransfer".to_string();
            let right_one = match &action_data.actionType {
                _xt => TxType::xTransfer,
                _ss => TxType::swap,
                _ => TxType::unknown,
            };
            tmp_scenario.txtype = right_one;
            //      println!("action_data.actionType: {:?}", action_data.actionType);
            //      println!(" tmp_scenario.txtype: {:?}",  tmp_scenario.txtype);
            //      println!(" right_one: {:?}",  right_one);
            tmp_scenario.amount = action_data.source.amount.clone();
            match action_data.source.assetId.to_owned() {
                Some(val) => tmp_scenario.assetid = val.into(),
                None => tmp_scenario.assetid = "0".to_string(),
            }
            //tmp_scenario.assetid = tmp_assetid.clone() as String;
            tmp_scenario.source_address = action_data.source.address.clone();
            tmp_scenario.dest_address = action_data.target.address.clone().expect("no dest_addr");
            tmp_scenario.source_chain = action_data.source.chain.clone();
            tmp_scenario.dest_chain = action_data.target.chain.clone();
            //  println!("tmp_scenario final: {:?}", tmp_scenario);
            alles.push(tmp_scenario);
        }
    }

    alles
}

pub fn generate_random_id() -> String {
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

/// verify that the user has provided a valid scenario_id, input sanitation
pub fn verify_scenario_id(scenario_id: String) -> bool {
    let input = scenario_id.as_str();
    input.len() == ID_LENGTH && input.chars().all(|c| CHARSET.contains(&(c as u8)))
}
