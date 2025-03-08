use serde::{Serialize, Deserialize}; // For serialization and deserialization
use std::hash::Hash; // For hashing
use crate::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property}; // import the SimpleHyperEdge and Property structs

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
// This struct represents a dual hyper edge in the graph database.
pub struct DualHyperEdge <T: Eq + Hash + ToString, K: Eq + Hash, V: Eq + Hash> {
    pub id: T, // Unique identifier for the dual hyper edge
    pub name: T, // Name of the dual hyper edge
    pub simple_hyper_edge: SimpleHyperEdge<T, K, V>, // The simple hyper edge associated with this dual hyper edge
    pub dual_properties: Vec<Property<K, V>>, // Properties associated with the dual hyper edge
    pub traversable: bool, // Indicates if the dual hyper edge is traversable
    pub head_hyper_nodes: Box<Vec<T>>, // List of head hyper nodes associated with this dual hyper edge
    pub tail_hyper_nodes: Option<Box<Vec<T>>> // Optional list of tail hyper nodes associated with this dual hyper edge
}