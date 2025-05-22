use reqwest::Client;
use serde_json::{json, Value};

pub struct GraphQLService {
    client: Client,
    endpoint: String,
}

impl GraphQLService {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn query(&self, query: &str) -> Result<Value, String> {
        let body = json!({
            "query": query,
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        let json_data: Value = serde_json::from_str(&response)
            .map_err(|e| e.to_string())?;

        Ok(json_data)
    }
}
