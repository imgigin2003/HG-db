use crate::hyper_edge::dto::layered_hypergraph_dto::{LayeredHypergraphCreateDto, LayeredHypergraphResponseDto};
use crate::hyper_edge::entity::h_graph::layered_hypergraph::LayeredHypergraph;
use crate::hyper_edge::mapper::simple_hyper_edge_mapper::{simple_from_create_dto, simple_to_response_dto};

pub fn layered_from_create_dto(dto: LayeredHypergraphCreateDto) -> LayeredHypergraph {
    let layers = dto.layers.into_iter().map(|layer_dtos| {
        layer_dtos.into_iter().map(simple_from_create_dto).collect()
    }).collect();

    LayeredHypergraph {
        id: dto.id,
        name: dto.name,
        layers,
        incidence_matrices: vec![],  
        transposed_matrices: vec![],
    }
}

pub fn layered_to_response_dto(entity: &LayeredHypergraph) -> LayeredHypergraphResponseDto {
    let layers = entity.layers.iter().map(|layer| {
        layer.iter().map(simple_to_response_dto).collect()
    }).collect();

    LayeredHypergraphResponseDto {
        id: entity.id.clone(),
        name: entity.name.clone(),
        layers,
        incidence_matrices: entity.incidence_matrices.clone(),
        transposed_matrices: entity.transposed_matrices.clone(),
    }
}