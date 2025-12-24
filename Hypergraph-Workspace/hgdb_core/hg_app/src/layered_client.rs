use serde::Serialize;
use std::collections::HashMap;
use reqwest::Client;

#[allow(dead_code)]
#[derive(Serialize)]
pub struct Hyperedge {
    pub nodes: Vec<String>,
    pub layer: i32,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct LayeredHypergraphRequest {
    pub hyperedges: HashMap<String, Hyperedge>,
}

#[allow(unused)]
pub async fn send_layered_hypergraph(
    data: LayeredHypergraphRequest,
) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let res = client
        .post("http://127.0.0.1:5000/api/render-layered")
        .json(&data)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}