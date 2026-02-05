use crate::hyper_edge::dto::light_hyper_edge_dto::LightHyperEdgeResponseDto;
use crate::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use crate::hyper_edge::mapper::{
    relationship_mapper::relationship_to_dto,
    simple_hyper_edge_mapper::simple_to_response_dto,
    structural_mapper::{structurals_to_dto, traverse_to_dto},
};

pub fn light_to_response_dto(entity: &LightHyperEdge<String, String, String>) -> LightHyperEdgeResponseDto {
    LightHyperEdgeResponseDto {
        id: entity.id.clone(),
        prime_simple_hyper_edge: simple_to_response_dto(&entity.prime_simple_hyper_edge),
        structural_properties: structurals_to_dto(&entity.structural_properties),
        relationship: relationship_to_dto(&entity.relationship),
        traverse: traverse_to_dto(&entity.traverse),
    }
}