use serde::{Serialize, Deserialize};
use crate::hyper_edge::dto::{
    simple_hyper_edge_dto::SimpleHyperEdgeResponseDto,
    structural_dto::{StructuralPropertyDto, TraverseDto},
    relationship_dto::RelationshipDto,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]   
pub struct LightHyperEdgeResponseDto {
    pub id: String,
    pub prime_simple_hyper_edge: SimpleHyperEdgeResponseDto,
    pub structural_properties: Vec<StructuralPropertyDto>,
    pub relationship: RelationshipDto,
    pub traverse: TraverseDto,
}