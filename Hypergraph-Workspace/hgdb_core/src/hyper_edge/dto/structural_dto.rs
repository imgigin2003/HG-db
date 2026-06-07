use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StructuralPropertyDto {
    pub address: Vec<String>,
    pub layer_index: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TraverseDto {
    pub path: Vec<String>,
    pub weight: Option<f64>,
}