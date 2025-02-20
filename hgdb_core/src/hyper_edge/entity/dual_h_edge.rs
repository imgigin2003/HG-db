use serde::{Serialize, Deserialize};
use std::hash::Hash;
use crate::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DualHyperEdge <T: Eq + Hash + ToString, K: Eq + Hash, V: Eq + Hash> {
    pub id: T,
    pub name: T,
    pub simple_hyper_edge: SimpleHyperEdge<T, K, V>,
    pub dual_properties: Vec<Property<K, V>>,
    pub traversable: bool,
    pub head_hyper_nodes: Box<Vec<String>>,
    pub tail_hyper_nodes: Box<Vec<String>>
}