use serde::{Deserialize, Serialize};
use std::hash::Hash;
use crate::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DualHyperEdge<T: Eq + Hash + Clone, K: Eq + Hash, V: Eq + Hash> {
    pub id: T,
    pub name: T,
    pub prime_simple_hyper_edge: SimpleHyperEdge<T, K, V>,
    pub dual_properties: Vec<Property<K, V>>,
    pub traversable: bool,
    pub head_hyper_nodes: Box<Vec<T>>,
    pub tail_hyper_nodes: Option<Box<Vec<T>>>,
    pub incidence_matrix: Vec<Vec<i8>>,
    pub transposed_matrix: Vec<Vec<i8>>,
}