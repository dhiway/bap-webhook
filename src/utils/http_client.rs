use reqwest::Client;
use serde_json::Value;

pub async fn post_json(url: &str, payload: Value) -> anyhow::Result<()> {
    let client = Client::new();
    let res = client.post(url).json(&payload).send().await?;

    if res.status().is_success() {
        println!("ok response");
        Ok(())
    } else {
        let status = res.status();
        let body = res.text().await?;
        Err(anyhow::anyhow!("Failed with status {}: {}", status, body))
    }
}
