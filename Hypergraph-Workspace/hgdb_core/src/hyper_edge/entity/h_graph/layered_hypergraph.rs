use serde::{Serialize, Deserialize};
use std::hash::Hash;
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LayeredHypergraph<T: Eq + Hash + Clone = String> {
    pub id: T,
    pub name: T,
    pub layers: Vec<Vec<SimpleHyperEdge<T, T, T>>>, 
    pub incidence_matrices: Vec<Vec<Vec<i8>>>,
    pub transposed_matrices: Vec<Vec<Vec<i8>>>,
}