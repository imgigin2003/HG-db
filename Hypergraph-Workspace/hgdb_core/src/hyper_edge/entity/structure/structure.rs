use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructuralProperty {
    pub address: Vec<String>,
    pub layer_index: Option<usize>,  
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Traverse {
    pub path: Vec<String>
}