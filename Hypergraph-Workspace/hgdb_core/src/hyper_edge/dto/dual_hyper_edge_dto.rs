use serde::{Serialize, Deserialize};
use crate::hyper_edge::dto::{property_dto::PropertyDto, simple_hyper_edge_dto::SimpleHyperEdgeResponseDto};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DualHyperEdgeCreateDto {
    pub id: String,
    pub name: String,
    pub prime_simple_hyper_edge_id: String,
    pub dual_properties: Vec<PropertyDto>,
    pub traversable: bool,
    pub head_hyper_node_ids: Vec<String>,
    pub tail_hyper_node_ids: Vec<String>,
    pub incidence_matrix: Vec<Vec<i8>>,
    pub transposed_matrix: Vec<Vec<i8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DualHyperEdgeUpdateDto {
    pub name: Option<String>,
    pub dual_properties: Option<Vec<PropertyDto>>,
    pub traversable: Option<bool>,
    pub head_hyper_node_ids: Option<Vec<String>>,
    pub tail_hyper_node_ids: Option<Vec<String>>,
    pub incidence_matrix: Option<Vec<Vec<i8>>>,
    pub transposed_matrix: Option<Vec<Vec<i8>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DualHyperEdgeResponseDto {
    pub id: String,
    pub name: String,
    pub prime_simple_hyper_edge: SimpleHyperEdgeResponseDto,
    pub dual_properties: Vec<PropertyDto>,
    pub traversable: bool,
    pub head_hyper_node_ids: Vec<String>,
    pub tail_hyper_node_ids: Vec<String>,
    pub incidence_matrix: Vec<Vec<i8>>,
    pub transposed_matrix: Vec<Vec<i8>>,
}