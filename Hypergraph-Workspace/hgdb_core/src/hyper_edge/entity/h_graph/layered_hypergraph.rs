use serde::{Serialize, Deserialize};
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LayeredHypergraph {
    pub id: String,
    pub name: String,
    pub layers: Vec<Vec<SimpleHyperEdge<String, String, String>>>, 
    pub incidence_matrices: Vec<Vec<Vec<i8>>>,
    pub transposed_matrices: Vec<Vec<Vec<i8>>>,
}