use serde::{Deserialize, Serialize};
use std::hash::Hash;
use crate::hyper_edge::entity::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::entity::relationship::relationship::Relationship;
use crate::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LightHyperEdge<T: Eq + Hash + std::fmt::Display, U: Eq + Hash, V: Eq + Hash> {
    pub id: T,
    pub simple_hyper_edge: SimpleHyperEdge<T, U, V>,
    pub structural_properties: Vec<StructuralProperty>,
    pub relationship: Relationship<T, U, V>,
    pub traverse: Traverse
}