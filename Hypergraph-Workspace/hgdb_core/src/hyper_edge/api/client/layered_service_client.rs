use reqwest::Client;
use serde_json;
use std::error::Error;

#[derive(Clone)]
pub struct LayeredServiceClient {
    client: Client,
    base_url: String,
}

impl LayeredServiceClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn send_for_visualization(
        &self,
        hypergraph: &crate::hyper_edge::dto::layered_hypergraph_dto::LayeredHypergraphResponseDto,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/api/visualize", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(hypergraph)
            .send()
            .await?
            .error_for_status()?;

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }

    pub async fn visualize_directly(
        &self,
        hypergraph: &crate::hyper_edge::dto::layered_hypergraph_dto::LayeredHypergraphResponseDto,
    ) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/api/visualize/direct", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(hypergraph)
            .send()
            .await?
            .error_for_status()?;

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }

    pub async fn health_check(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/health", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
