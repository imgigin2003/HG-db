use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::dto::simple_hyper_edge_dto::*;
use crate::hyper_edge::mapper::property_mapper::*;

pub fn simple_to_response_dto(entity: &SimpleHyperEdge<String, String, String>) -> SimpleHyperEdgeResponseDto {
    let head_ids: Vec<String> = entity
        .head_hyper_nodes
        .as_deref()
        .unwrap_or(&vec![])
        .iter()
        .map(|node| node.id.clone())
        .collect();

    let tail_ids: Vec<String> = entity
        .tail_hyper_nodes
        .as_deref()
        .unwrap_or(&vec![])
        .iter()
        .map(|node| node.id.clone())
        .collect();

    SimpleHyperEdgeResponseDto {
        id: entity.id.clone(),
        name: entity.name.clone(),
        main_properties: properties_to_dto(&entity.main_properties),
        traversable: entity.traversable,
        directed: entity.directed,
        head_hyper_node_ids: head_ids,
        tail_hyper_node_ids: tail_ids,
        incidence_matrix: entity.incidence_matrix.clone(),
    }
}

pub fn simple_from_create_dto(dto: SimpleHyperEdgeCreateDto) -> SimpleHyperEdge<String, String, String> {
    let head_nodes = dto.head_hyper_node_ids.map(|ids| {
        Box::new(ids.into_iter().map(|id| SimpleHyperEdge {
            id: id.clone(),
            name: id.clone(), 
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
            incidence_matrix: vec![],
        }).collect())
    });

    let tail_nodes = dto.tail_hyper_node_ids.map(|ids| {
        Box::new(ids.into_iter().map(|id| SimpleHyperEdge {
            id: id.clone(),
            name: id.clone(),
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
            incidence_matrix: vec![],
        }).collect())
    });

    SimpleHyperEdge {
        id: dto.id,
        name: dto.name,
        main_properties: properties_from_dto(&dto.main_properties),
        traversable: dto.traversable,
        directed: dto.directed,
        head_hyper_nodes: head_nodes,
        tail_hyper_nodes: tail_nodes,
        incidence_matrix: dto.incidence_matrix,
    }
}