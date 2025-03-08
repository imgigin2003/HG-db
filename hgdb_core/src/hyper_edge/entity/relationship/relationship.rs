use serde::{Deserialize, Serialize}; // For serialization and deserialization

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Represents a relationship between two nodes in a hypergraph
pub struct Relationship<T, U, V> {
    pub node_1: T, // The first node in the relationship
    pub node_2: U, // The second node in the relationship
    pub directed: bool, // Indicates if the relationship is directed
    pub edge_properties: Vec<V> // Properties associated with the relationship
}