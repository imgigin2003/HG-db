use crate::hyper_edge::dto::dual_hyper_edge_dto::*;
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge;
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::mapper::property_mapper::*;
use crate::hyper_edge::mapper::simple_hyper_edge_mapper::simple_to_response_dto;

pub fn dual_to_response_dto(entity: &DualHyperEdge<String, String, String>) -> DualHyperEdgeResponseDto {
    DualHyperEdgeResponseDto {
        id: entity.id.clone(),
        name: entity.name.clone(),
        prime_simple_hyper_edge: simple_to_response_dto(&entity.prime_simple_hyper_edge),
        dual_properties: properties_to_dto(&entity.dual_properties),
        traversable: entity.traversable,
        head_hyper_node_ids: entity.head_hyper_nodes.clone(),
        tail_hyper_node_ids: entity.tail_hyper_nodes.clone(),
        incidence_matrix: entity.incidence_matrix.clone(),
        transposed_matrix: entity.transposed_matrix.clone(),
    }
}

pub fn dual_from_create_dto(
    dto: DualHyperEdgeCreateDto,
    prime: SimpleHyperEdge<String, String, String>,
) -> DualHyperEdge<String, String, String> {
    DualHyperEdge {
        id: dto.id,
        name: dto.name,
        prime_simple_hyper_edge: prime,
        dual_properties: properties_from_dto(&dto.dual_properties),
        traversable: dto.traversable,
        head_hyper_nodes: dto.head_hyper_node_ids,
        tail_hyper_nodes: dto.tail_hyper_node_ids,
        incidence_matrix: dto.incidence_matrix,
        transposed_matrix: dto.transposed_matrix,
    }
}