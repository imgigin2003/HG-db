use crate::hyper_edge::entity::h_graph::layered_hypergraph::LayeredHypergraph;
use crate::hyper_edge::dto::layered_hypergraph_dto::{LayeredHypergraphCreateDto, LayeredHypergraphResponseDto};
use crate::hyper_edge::mapper::layered_hypergraph_mapper;
use crate::hyper_edge::repository::Repository;
use crate::hyper_edge::repository::h_graph_repository::LayeredHypergraphRepository;
use std::sync::Arc;
use std::error::Error;

pub struct LayeredHypergraphService {
    repository: Arc<LayeredHypergraphRepository>,
}

impl LayeredHypergraphService {
    pub fn new(repository: Arc<LayeredHypergraphRepository>) -> Self {
        Self { repository }
    }
    
    pub async fn create_layered_hypergraph(
        &self,
        dto: LayeredHypergraphCreateDto,
    ) -> Result<LayeredHypergraphResponseDto, Box<dyn Error>> {
        // Validate the DTO
        self.validate_layered_hypergraph(&dto)?;
        
        // Convert DTO to entity
        let entity = layered_hypergraph_mapper::layered_from_create_dto(dto)?;
        
        // Calculate matrices
        let entity_with_matrices = self.calculate_matrices(entity);
        
        // Save to RocksDB
        self.repository.create(&entity_with_matrices.id, entity_with_matrices.clone())?;
        
        // Convert to response DTO
        let response_dto = layered_hypergraph_mapper::layered_to_response_dto(&entity_with_matrices);
        
        Ok(response_dto)
    }
    
    pub async fn get_layered_hypergraph(
        &self,
        id: &str,
    ) -> Option<LayeredHypergraphResponseDto> {
        match self.repository.get_by_key(id) {
            Ok(Some(entity)) => {
                Some(layered_hypergraph_mapper::layered_to_response_dto(&entity))
            }
            _ => None,
        }
    }
    
    pub async fn update_layered_hypergraph(
        &self,
        id: &str,
        dto: LayeredHypergraphCreateDto,
    ) -> Result<LayeredHypergraphResponseDto, Box<dyn Error>> {
        // Check if exists
        if self.get_layered_hypergraph(id).await.is_none() {
            return Err("Layered hypergraph not found".into());
        }
        
        // Validate
        self.validate_layered_hypergraph(&dto)?;
        
        // Update in database
        let entity = layered_hypergraph_mapper::layered_from_create_dto(dto)?;
        let entity_with_matrices = self.calculate_matrices(entity);
        
        self.repository.update(id, &entity_with_matrices)?;
        
        let response_dto = layered_hypergraph_mapper::layered_to_response_dto(&entity_with_matrices);
        
        Ok(response_dto)
    }
    
    pub async fn delete_layered_hypergraph(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn Error>> {
        self.repository.delete(id)?;
        Ok(())
    }
    
    pub async fn list_layered_hypergraphs(
        &self,
    ) -> Result<Vec<LayeredHypergraphResponseDto>, Box<dyn Error>> {
        let entities = self.repository.get_all()?;
        let response_dtos: Vec<LayeredHypergraphResponseDto> = entities
            .iter()
            .map(|entity| layered_hypergraph_mapper::layered_to_response_dto(entity))
            .collect();
        Ok(response_dtos)
    }
    
    fn validate_layered_hypergraph(
        &self,
        dto: &LayeredHypergraphCreateDto,
    ) -> Result<(), Box<dyn Error>> {
        if dto.id.is_empty() {
            return Err("ID cannot be empty".into());
        }
        
        if dto.name.is_empty() {
            return Err("Name cannot be empty".into());
        }
        
        if dto.layers.is_empty() {
            return Err("At least one layer is required".into());
        }
        
        // Validate each layer
        for (layer_index, layer) in dto.layers.iter().enumerate() {
            if layer.is_empty() {
                return Err(format!("Layer {} cannot be empty", layer_index).into());
            }
            
            for edge in layer {
                if edge.head_hyper_node_ids.is_none() && edge.tail_hyper_node_ids.is_none() {
                    return Err(format!("Edge {} must have at least head or tail nodes", edge.id).into());
                }
            }
        }
        
        Ok(())
    }
    
    fn calculate_matrices(
        &self,
        mut hypergraph: LayeredHypergraph,
    ) -> LayeredHypergraph {
        let mut incidence_matrices = Vec::new();
        let mut transposed_matrices = Vec::new();
        
        for layer in &hypergraph.layers {
            let matrix = self.calculate_layer_incidence_matrix(layer);
            let transposed = self.transpose_matrix(&matrix);
            
            incidence_matrices.push(matrix);
            transposed_matrices.push(transposed);
        }
        
        hypergraph.incidence_matrices = incidence_matrices;
        hypergraph.transposed_matrices = transposed_matrices;
        
        hypergraph
    }
    
    fn calculate_layer_incidence_matrix(
        &self,
        layer: &[crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge<String, String, String>],
    ) -> Vec<Vec<i8>> {
        let mut all_nodes = Vec::new();
        
        for edge in layer {
            if let Some(head_nodes) = &edge.head_hyper_nodes {
                for node in head_nodes {
                    if !all_nodes.contains(node) {
                        all_nodes.push(node.clone());
                    }
                }
            }
            
            if let Some(tail_nodes) = &edge.tail_hyper_nodes {
                for node in tail_nodes {
                    if !all_nodes.contains(node) {
                        all_nodes.push(node.clone());
                    }
                }
            }
        }
        
        all_nodes.sort();
        
        // ایجاد ماتریس incidence
        let mut incidence_matrix = vec![vec![0; layer.len()]; all_nodes.len()];
        
        for (edge_idx, edge) in layer.iter().enumerate() {
            for (node_idx, node) in all_nodes.iter().enumerate() {
                let mut value = 0;
                
                // چک کردن head nodes
                if let Some(head_nodes) = &edge.head_hyper_nodes {
                    if head_nodes.contains(node) {
                        value += 1;
                    }
                }
                
                // چک کردن tail nodes
                if let Some(tail_nodes) = &edge.tail_hyper_nodes {
                    if tail_nodes.contains(node) {
                        value += 2;
                    }
                }
                
                incidence_matrix[node_idx][edge_idx] = value;
            }
        }
        
        incidence_matrix
    }
    
    fn transpose_matrix(&self, matrix: &[Vec<i8>]) -> Vec<Vec<i8>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut transposed = vec![vec![0; rows]; cols];
        
        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = matrix[i][j];
            }
        }
        
        transposed
    }
}