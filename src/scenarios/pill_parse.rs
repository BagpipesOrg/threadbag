use crate::error::Error;
use crate::scenarios::scenario_types::{ChainNode, FormData, HTTPNode, HTTP_NODE_FORMDATA};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// parse pill nodes
#[derive(Debug, Deserialize, Serialize)] // , Clone
pub struct PillParse {
    pub input: String,
}

pub fn process_node(node: &mut HTTPNode, webhook_loot: &HashMap<String, Value>) {
    if let Some(form_data) = &mut node.formData {
        process_http_form_data(form_data, webhook_loot);
    }
}

pub fn process_chain_node(node: &mut ChainNode, webhook_loot: &HashMap<String, Value>) {
    if let Some(form_data) = &mut node.formData {
        process_form_data(form_data, webhook_loot);
    }
}

pub fn process_form_data(form_data: &mut FormData, webhook_loot: &HashMap<String, Value>) {
    process_field(&mut form_data.chain, webhook_loot);
    //  process_field(&mut form_data.asset, webhook_loot);
    process_field(&mut form_data.methodInput, webhook_loot);
    process_field(&mut form_data.address, webhook_loot);
    process_field(&mut form_data.amount, webhook_loot);
    //process_field(&mut form_data.delay, webhook_loot);
    //process_field(&mut form_data.contact, webhook_loot);
    //process_field(&mut form_data.action, webhook_loot);
    //   process_field(&mut form_data.actionData, webhook_loot);
    process_field(&mut form_data.serializeUrl, webhook_loot);
    process_field(&mut form_data.parseResponse, webhook_loot);
    process_field(&mut form_data.shareCookies, webhook_loot);
    process_field(&mut form_data.rejectUnverifiedCertificates, webhook_loot);
    process_field(&mut form_data.followRedirects, webhook_loot);
    process_field(&mut form_data.followAllRedirects, webhook_loot);
    process_field(&mut form_data.requestCompressedContent, webhook_loot);
    process_field(&mut form_data.useMutualTLS, webhook_loot);
    process_field(&mut form_data.evaluateErrors, webhook_loot);
    process_field(&mut form_data.url, webhook_loot);
    process_field(&mut form_data.method, webhook_loot);
    process_field(&mut form_data.selectedChain, webhook_loot);
    process_field(&mut form_data.selectedPallet, webhook_loot);
    process_field(&mut form_data.connectionType, webhook_loot);
    process_field(&mut form_data.uuid, webhook_loot);
    process_field(&mut form_data.webhookName, webhook_loot);
    //  process_field(&mut form_data.params.as_mut().map(|params| &mut params["param_key"]), webhook_loot); // Adjust logic for HashMap processing if necessary
    process_field(&mut form_data.name, webhook_loot);
}

//fn process_asset_field(field: &mut Option<As>) {}

fn process_field(field: &mut Option<String>, webhook_loot: &HashMap<String, Value>) {
    if let Some(field_value) = field.as_ref() {
        match extract_pill(field_value.to_owned()) {
            Ok(value) => {
                if let Some(Value::String(converted_value)) = webhook_loot.get(&value) {
                    *field = Some(converted_value.clone());
                } else {
                    println!("Could not get pill value");
                }
            }
            Err(_) => {
    //        println!("No HTML content found in field");
            }
        }
    }

    if let Some(value) = field.as_mut() {
        if let Some(replacement) = webhook_loot.get(value) {
            *value = replacement.as_str().unwrap_or(value).to_string();
        }
    }
}
fn process_field_simple(field: &mut String, webhook_loot: &HashMap<String, Value>) {
    match extract_pill(field.to_string()) {
        Ok(value) => {
            //  println!("Extracted pill as: {}", value);
            if let Some(Value::String(converted_value)) = webhook_loot.get(&value) {
                //     println!("Converted value to: {:?}", converted_value);
                *field = converted_value.clone();
            } else {
                //       println!("Could not get pill value");
            }
        }
        Err(_) => {
            //     println!("No HTML content found in field");
        }
    }
}

pub fn process_http_form_data(
    form_data: &mut HTTP_NODE_FORMDATA,
    webhook_loot: &HashMap<String, Value>,
) {
    process_field_simple(&mut form_data.serializeUrl, webhook_loot);
    process_field_simple(&mut form_data.parseResponse, webhook_loot);
    process_field_simple(&mut form_data.shareCookies, webhook_loot);
    process_field_simple(&mut form_data.rejectUnverifiedCertificates, webhook_loot);
    process_field_simple(&mut form_data.followRedirects, webhook_loot);
    process_field_simple(&mut form_data.followAllRedirects, webhook_loot);
    process_field_simple(&mut form_data.requestCompressedContent, webhook_loot);
    process_field_simple(&mut form_data.useMutualTLS, webhook_loot);
    process_field_simple(&mut form_data.evaluateErrors, webhook_loot);
    process_field_simple(&mut form_data.url, webhook_loot);
    process_field_simple(&mut form_data.method, webhook_loot);
    process_field_simple(&mut form_data.connectionType, webhook_loot);
}

/// regex out the column name, error if not found
pub fn extract_pill(input: String) -> Result<String, Error> {
    let re =
        Regex::new(r#"\<span\sclass="pill"\sdata-id="eventData\.content\.([a-z-A-Z-0-9]{0,})"#)
            .unwrap();
    match re.captures(input.as_str()) {
        Some(Value) => {
            let first = Value.get(1).expect("could not extract pill node").as_str();
            let out = format!("{}", first);
            return Ok(out);
        }

        _ => return Err(Error::PillDataError),
    }
}

/*
// one field
fn process_field(field: &mut String, webhook_loot: &HashMap<String, Value>) {
    match extract_pill(field.to_string()) {
        Ok(value) => {
            //  println!("Extracted pill as: {}", value);
            if let Some(Value::String(converted_value)) = webhook_loot.get(&value) {
                //     println!("Converted value to: {:?}", converted_value);
                *field = converted_value.clone();
            } else {
                //       println!("Could not get pill value");
            }
        }
        Err(_) => {
            //     println!("No HTML content found in field");
        }
    }
}
*/

impl PillParse {
    pub fn new(input: String) -> Self {
        PillParse { input }
    }

    pub fn contains_process() {
        let pill_regex = Regex::new(
            r#"<span[^>]*data-id="([^"]+)"[^>]*data-nodeindex="(\d+)"[^>]*>([^<]+)</span>"#,
        )
        .unwrap();
    }

    pub fn sanitize_value(value: &str) -> String {
        let sanitized_value = Regex::new(r#"<div[^>]*>(.*?)</div>"#)
            .unwrap()
            .replace_all(value, "$1");
        let sanitized_value = Regex::new(r#"<span(?![^>]*class="pill")[^>]*>(.*?)</span>"#)
            .unwrap()
            .replace_all(&sanitized_value, "$1");
        sanitized_value.to_string()
    }

    fn rebuild_string(value: &str) -> String {
        value
            .chars()
            .filter(|&c| c as u32 >= 32 && c as u32 <= 126)
            .collect()
    }

    fn is_valid_base58(value: &str) -> bool {
        true //validator::validate(value, "base58").is_ok()
    }

    fn log_char_codes(value: &str) -> Vec<(char, u32, usize)> {
        value
            .chars()
            .enumerate()
            .map(|(i, c)| (c, c as u32, i))
            .collect()
    }

    pub fn contains_pill() {}
}
