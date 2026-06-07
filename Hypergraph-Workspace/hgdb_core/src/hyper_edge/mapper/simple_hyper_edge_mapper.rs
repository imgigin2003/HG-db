use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::dto::simple_hyper_edge_dto::*;
use crate::hyper_edge::mapper::property_mapper::*;
use std::error::Error;

pub fn simple_to_response_dto(entity: &SimpleHyperEdge<String, String, String>) -> SimpleHyperEdgeResponseDto {
    SimpleHyperEdgeResponseDto {
        id: entity.id.clone(),
        name: entity.name.clone(),
        main_properties: properties_to_dto(&entity.main_properties),
        traversable: entity.traversable,
        directed: entity.directed,
        head_hyper_node_ids: entity.head_hyper_nodes.clone().unwrap_or_default(),
        tail_hyper_node_ids: entity.tail_hyper_nodes.clone().unwrap_or_default(),
        incidence_matrix: entity.incidence_matrix.clone(),
    }
}

pub fn simple_from_create_dto(dto: SimpleHyperEdgeCreateDto) -> Result<SimpleHyperEdge<String, String, String>, Box<dyn Error>> {
    Ok(SimpleHyperEdge {
        id: dto.id,
        name: dto.name,
        main_properties: properties_from_dto(&dto.main_properties),
        traversable: dto.traversable,
        directed: dto.directed,
        head_hyper_nodes: dto.head_hyper_node_ids,
        tail_hyper_nodes: dto.tail_hyper_node_ids,
        incidence_matrix: dto.incidence_matrix,
    })
}