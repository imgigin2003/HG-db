use crate::hyper_edge::entity::h_graph::layered_hypergraph::LayeredHypergraph;
use crate::hyper_edge::dto::layered_hypergraph_dto::{LayeredHypergraphCreateDto, LayeredHypergraphResponseDto};
use crate::hyper_edge::mapper::simple_hyper_edge_mapper;
use std::error::Error;

pub fn layered_to_response_dto(entity: &LayeredHypergraph) -> LayeredHypergraphResponseDto {
    let layers: Vec<Vec<_>> = entity.layers
        .iter()
        .map(|layer| {
            layer.iter()
                .map(|edge| simple_hyper_edge_mapper::simple_to_response_dto(edge))
                .collect()
        })
        .collect();
    
    LayeredHypergraphResponseDto {
        id: entity.id.clone(),
        name: entity.name.clone(),
        layers,
        incidence_matrices: entity.incidence_matrices.clone(),
        transposed_matrices: entity.transposed_matrices.clone(),
    }
}

pub fn layered_from_create_dto(dto: LayeredHypergraphCreateDto) -> Result<LayeredHypergraph, Box<dyn Error>> {
    let mut layers = Vec::new();
    
    for layer_dto in dto.layers {
        let mut layer = Vec::new();
        
        for edge_dto in layer_dto {
            layer.push(simple_hyper_edge_mapper::simple_from_create_dto(edge_dto)?);
        }
        
        layers.push(layer);
    }
    
    Ok(LayeredHypergraph {
        id: dto.id,
        name: dto.name,
        layers,
        incidence_matrices: Vec::new(),
        transposed_matrices: Vec::new(),
    })
}