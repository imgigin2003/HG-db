use crate::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use crate::hyper_edge::entity::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::entity::dual_h_edge::DualHyperEdge;
use std::error::Error;

pub struct DualHyperEdgeService<'a> {
    repository: &'a SimpleHyperEdgeRepository,
}

impl<'a> DualHyperEdgeService<'a> {
    pub fn new(repository: &'a SimpleHyperEdgeRepository) -> Self {
        DualHyperEdgeService { repository }
    }

    pub fn create_dual_h_edge(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let simple_h_edge = self.repository.get_by_key(id)?;
    
        if let Some(original_edge) = simple_h_edge {
            let mut nodes_set = original_edge.head_hyper_nodes.clone();
            nodes_set.extend_from_slice(&original_edge.tail_hyper_nodes);
    
            let incidence_matrix = self.create_incidence_matrix(&nodes_set, &original_edge);
            let transposed_matrix = self.transpose_matrix(&incidence_matrix);
    
            // ✅ Debugging: Print matrices in a readable format
            println!("🔢 Original Incidence Matrix:");
            self.print_matrix(&incidence_matrix);
    
            println!("🔄 Transposed Matrix:");
            self.print_matrix(&transposed_matrix);
    
            let dual_edge_id = format!("dual_{}", id);
    
            let dual_edge = DualHyperEdge {
                id: dual_edge_id.clone(),
                name: format!("Dual of {}", original_edge.name),
                simple_hyper_edge: original_edge.clone(),
                dual_properties: original_edge.main_properties.clone(),
                traversable: original_edge.traversable,
                head_hyper_nodes: original_edge.head_hyper_nodes.clone(),
                tail_hyper_nodes: original_edge.tail_hyper_nodes.clone(),
            };
    
            println!("🛠 Attempting to save Dual Hyperedge with Key: {}", dual_edge.id);
            self.repository.save_dual(dual_edge)?;

        }
    
        Ok(())
    }            

    // Simulate matrix creation based on head and tail nodes
    pub fn create_incidence_matrix(
        &self,
        nodes: &Vec<String>,
        original_edge: &SimpleHyperEdge<String, String, String>,
    ) -> Vec<Vec<bool>> {
        let mut matrix = Vec::new();

        // Create rows for each node (just an example)
        for node in nodes {
            let mut row = vec![false; 1]; // Matrix is 1-dimensional for simplicity
            // Here, you could fill in the row based on your logic
            // Example: if node is in head or tail of the edge, mark it
            if original_edge.head_hyper_nodes.contains(node) {
                row[0] = true;
            }
            matrix.push(row);
        }

        matrix
    }

    // Function to transpose the matrix
    pub fn transpose_matrix(&self, matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![]; // Return empty if matrix is empty
        }
    
        let rows = matrix.len();
        let cols = matrix[0].len();
    
        let mut transposed = vec![vec![false; rows]; cols]; // Flip row/column sizes
    
        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = matrix[i][j]; // Swap positions
            }
        }
    
        transposed
    }    

    pub fn print_matrix(&self, matrix: &Vec<Vec<bool>>) {
        println!("🔢 Matrix [{}x{}]:", matrix.len(), if matrix.is_empty() { 0 } else { matrix[0].len() });
        for row in matrix {
            let row_str: String = row.iter()
                .map(|&val| if val { "1" } else { "0" })
                .collect::<Vec<&str>>()
                .join(" ");
            println!("[ {} ]", row_str);
        }
    } 
}
