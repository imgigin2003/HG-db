use serde::{Serialize, Deserialize};
use crate::hyper_edge::dto::property_dto::PropertyDto;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SimpleHyperEdgeCreateDto {
    pub id: String,
    pub name: String,
    pub main_properties: Vec<PropertyDto>,
    pub traversable: bool,
    pub directed: bool,
    pub head_hyper_node_ids: Option<Vec<String>>,
    pub tail_hyper_node_ids: Option<Vec<String>>,
    pub incidence_matrix: Vec<Vec<i8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SimpleHyperEdgeUpdateDto {
    pub name: Option<String>,
    pub main_properties: Option<Vec<PropertyDto>>,
    pub traversable: Option<bool>,
    pub directed: Option<bool>,
    pub head_hyper_node_ids: Option<Vec<String>>,
    pub tail_hyper_node_ids: Option<Vec<String>>,
    pub incidence_matrix: Option<Vec<Vec<i8>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimpleHyperEdgeResponseDto {
    pub id: String,
    pub name: String,
    pub main_properties: Vec<PropertyDto>,
    pub traversable: bool,
    pub directed: bool,
    pub head_hyper_node_ids: Vec<String>,
    pub tail_hyper_node_ids: Vec<String>,
    pub incidence_matrix: Vec<Vec<i8>>,
}