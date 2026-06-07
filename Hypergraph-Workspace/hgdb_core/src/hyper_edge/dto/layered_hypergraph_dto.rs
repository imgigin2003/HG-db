use serde::{Serialize, Deserialize};
use crate::hyper_edge::dto::simple_hyper_edge_dto::{SimpleHyperEdgeCreateDto, SimpleHyperEdgeResponseDto};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LayeredHypergraphCreateDto {
    pub id: String,
    pub name: String,
    pub layers: Vec<Vec<SimpleHyperEdgeCreateDto>>, 
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LayeredHypergraphResponseDto {
    pub id: String,
    pub name: String,
    pub layers: Vec<Vec<SimpleHyperEdgeResponseDto>>,
    pub incidence_matrices: Vec<Vec<Vec<i8>>>,  
    pub transposed_matrices: Vec<Vec<Vec<i8>>>,
}