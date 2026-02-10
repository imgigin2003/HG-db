use crate::hyper_edge::dto::layered_hypergraph_dto::LayeredHypergraphCreateDto;
use crate::hyper_edge::dto::layered_visualization_request::{
    LayeredVisualizationRequestDto, SimpleEdgeInputDto,
};
use crate::hyper_edge::dto::property_dto::{PropertyDto, PropertyTypeDto};
use crate::hyper_edge::dto::simple_hyper_edge_dto::SimpleHyperEdgeCreateDto;
use std::collections::HashMap;
use std::error::Error;

pub fn transform_visualization_request(
    request: LayeredVisualizationRequestDto,
) -> Result<LayeredHypergraphCreateDto, Box<dyn Error>> {
    // Group edges by layer
    let mut layers_map: HashMap<i32, Vec<SimpleEdgeInputDto>> = HashMap::new();

    for edge in request.edges {
        layers_map
            .entry(edge.layer)
            .or_insert_with(Vec::new)
            .push(edge);
    }

    // Convert to layers
    let mut layers: Vec<Vec<SimpleHyperEdgeCreateDto>> = Vec::new();
    let mut layer_keys: Vec<&i32> = layers_map.keys().collect();
    layer_keys.sort();

    for &layer_key in layer_keys.iter() {
        let edges = layers_map.get(layer_key).unwrap();
        let mut layer_edges: Vec<SimpleHyperEdgeCreateDto> = Vec::new();

        for edge in edges {
            match transform_edge_input(edge) {
                Ok(transformed_edge) => layer_edges.push(transformed_edge),
                Err(e) => return Err(format!("Error transforming edge {}: {}", edge.id, e).into()),
            }
        }

        layers.push(layer_edges);
    }

    // Create hypergraph DTO
    let hypergraph_id = if request.hypergraph_id.is_empty() {
        generate_hypergraph_id()
    } else {
        request.hypergraph_id
    };

    let hypergraph_name = if request.hypergraph_name.is_empty() {
        format!("Layered Hypergraph {}", hypergraph_id)
    } else {
        request.hypergraph_name
    };

    Ok(LayeredHypergraphCreateDto {
        id: hypergraph_id,
        name: hypergraph_name,
        layers,
    })
}

fn transform_edge_input(
    edge: &SimpleEdgeInputDto,
) -> Result<SimpleHyperEdgeCreateDto, Box<dyn Error>> {
    // Convert head nodes
    let head_hyper_node_ids = edge
        .head_hyper_nodes
        .as_ref()
        .map(|nodes| nodes.iter().map(|n| n.id.clone()).collect())
        .unwrap_or_else(Vec::new);

    // Convert tail nodes
    let tail_hyper_node_ids = edge
        .tail_hyper_nodes
        .as_ref()
        .map(|nodes| nodes.iter().map(|n| n.id.clone()).collect())
        .unwrap_or_else(Vec::new);

    // Convert properties
    let main_properties: Vec<PropertyDto> = edge
        .main_properties
        .iter()
        .map(|prop| {
            // Convert string to PropertyTypeDto
            let p_type = match prop.p_type.as_str() {
                "Simple" => PropertyTypeDto::Simple,
                "Main" => PropertyTypeDto::Main,
                "Structure" => PropertyTypeDto::Structure,
                "ExtraInfo" => PropertyTypeDto::ExtraInfo,
                _ => PropertyTypeDto::Simple, // Default fallback
            };

            PropertyDto {
                key: prop.key.clone(),
                p_type,
                value: prop.value.clone(),
            }
        })
        .collect();

    Ok(SimpleHyperEdgeCreateDto {
        id: edge.id.clone(),
        name: edge.name.clone(),
        main_properties,
        traversable: edge.traversable,
        directed: edge.directed,
        head_hyper_node_ids: if head_hyper_node_ids.is_empty() {
            None
        } else {
            Some(head_hyper_node_ids)
        },
        tail_hyper_node_ids: if tail_hyper_node_ids.is_empty() {
            None
        } else {
            Some(tail_hyper_node_ids)
        },
        incidence_matrix: Vec::new(), // Will be calculated later
    })
}

fn generate_hypergraph_id() -> String {
    use chrono::Utc;
    let timestamp = Utc::now().timestamp();
    format!("hypergraph-{}", timestamp)
}
