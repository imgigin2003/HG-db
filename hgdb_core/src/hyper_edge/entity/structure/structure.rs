use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StructuralProperty {
    pub address: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Traverse {
    pub path: Vec<String>
}