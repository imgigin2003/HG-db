use serde::{Deserialize, Serialize}; // For serialization and deserialization
use std::hash::Hash; // For hashing
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge; // import simple hyper edge
use crate::hyper_edge::entity::relationship::relationship::Relationship; // import relationship
use crate::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse}; // import structural properties and traverse

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A struct representing a light hyper edge in a graph database
pub struct LightHyperEdge<T: Eq + Hash + std::fmt::Display, U: Eq + Hash, V: Eq + Hash> {
    pub id: T, // Unique identifier for the hyper edge
    pub simple_hyper_edge: SimpleHyperEdge<T, U, V>, // The simple hyper edge associated with this light hyper edge
    pub structural_properties: Vec<StructuralProperty>, // Properties that define the structure of the hyper edge
    pub relationship: Relationship<T, U, V>, // The relationship associated with this hyper edge
    pub traverse: Traverse // The traversal information for this hyper edge
}