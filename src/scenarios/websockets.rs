use crate::error::Error;
use reqwest::Client;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

// download the latest data sent to webhook
pub async fn latest_webhookevents(uuid: String) -> Result<HashMap<String, JsonValue>, Error> {
    println!("getting latest webhookevents");
    let url = format!(
        "https://webhook.site/token/{}/request/latest/raw?sorting=newest",
        uuid
    ); // test: 885d929c-2016-46ed-bb11-9ee59f784b12
    let client = Client::new();

    let response = client
        .get(url)
        .header("accept", "application/json")
        .header("api-key", "bde29c1a-16aa-4a3e-b6ff-8a0644f3b6c3")
        .send()
        .await?;

    let _status = response.status();
    let body = response.json::<HashMap<String, JsonValue>>().await?;
    Ok(body)
}
