use crate::hyper_edge::dto::hyper_edge_dto::HyperEdgeResponseDto;
use crate::hyper_edge::entity::h_edge::hyper_edge::HyperEdge;
use crate::hyper_edge::mapper::light_hyper_edge_mapper::light_to_response_dto;

pub fn hyper_edge_to_response_dto(entity: &HyperEdge<String, String, String>) -> HyperEdgeResponseDto {
    HyperEdgeResponseDto {
        id: entity.id.clone(),
        light_hyper_edge: light_to_response_dto(&entity.light_hyper_edge),
        attachments: entity
            .attachments
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
    }
}