use serde::{Deserialize, Serialize}; // For serialization and deserialization

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Represents a structure with a name and an optional description
pub struct StructuralProperty {
    pub address: Vec<String> // The address of the structure
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Represents a structure with a name and an optional description
pub struct Traverse {
    pub path: Vec<String> // The path of the structure
}