use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimpleEdgeInputDto {
    pub id: String,
    pub name: String,
    pub main_properties: Vec<PropertyInputDto>,
    pub traversable: bool,
    pub directed: bool,
    pub head_hyper_nodes: Option<Vec<NodeInputDto>>,
    pub tail_hyper_nodes: Option<Vec<NodeInputDto>>,
    pub layer: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeInputDto {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PropertyInputDto {
    pub key: String,
    #[serde(rename = "p_type")]
    pub p_type: String,
    pub value: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LayeredVisualizationRequestDto {
    pub edges: Vec<SimpleEdgeInputDto>,
    #[serde(default)]
    pub hypergraph_id: String,
    #[serde(default)]
    pub hypergraph_name: String,
}
