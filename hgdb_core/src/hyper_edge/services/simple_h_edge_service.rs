use crate::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge;
use std::error::Error;

pub struct DualHyperEdgeService<'a> {
    repository: &'a SimpleHyperEdgeRepository,
}

impl<'a> DualHyperEdgeService<'a> {
    pub fn new(repository: &'a SimpleHyperEdgeRepository) -> Self {
        DualHyperEdgeService { repository }
    }

    // method to create the dual edge based on the simple edge
    pub fn create_dual_h_edge(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let simple_h_edge = self.repository.get_by_key(id)?;
        let original_edge = simple_h_edge.ok_or_else(|| {
            let msg = format!("No SimpleHyperEdge found for key: {}", id);
            eprintln!("❌ {}", msg);
            Box::<dyn Error>::from(msg)
        })?;

        let mut nodes_set = original_edge.head_hyper_nodes.as_ref().clone();
        if let Some(tail_nodes) = &original_edge.tail_hyper_nodes {
            nodes_set.extend_from_slice(tail_nodes);
        }
        println!("🧩 Nodes Set: {:?}", nodes_set);

        let incidence_matrix = self.create_incidence_matrix(&nodes_set, &original_edge);
        let transposed_matrix = self.transpose_matrix(&incidence_matrix);
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
            head_hyper_nodes: Box::new(original_edge.head_hyper_nodes.as_ref().clone()),
            tail_hyper_nodes: original_edge.tail_hyper_nodes.clone(),
            incidence_matrix,
            transposed_matrix,
        };

        println!("🛠 Saving Dual Hyperedge with Key: {}", dual_edge.id);
        self.repository.save_dual(dual_edge)?;
        println!("✅ Dual Hyperedge saved successfully");

        Ok(())
    }            

    // Simulate matrix creation based on head and tail nodes
    pub fn create_incidence_matrix<T: ToString>(
        &self,
        nodes: &[T],
        original_edge: &SimpleHyperEdge<String, String, String>,
    ) -> Vec<Vec<i8>> {
        let mut matrix = vec![vec![0i8; 1]; nodes.len()]; // Single column for one edge, initialized to 0

        for (i, node) in nodes.iter().enumerate() {
            let node_str = node.to_string();
            let is_in_head = original_edge.head_hyper_nodes.contains(&node_str);
            let is_in_tail = original_edge.tail_hyper_nodes.as_ref()
                .map_or(false, |tail| tail.contains(&node_str));

            matrix[i][0] = match (is_in_head, is_in_tail) {
                (true, false) => 1,  // Weight for head node
                (false, true) => 2,  // Weight for tail node
                (true, true) => 3,   // Weight if in both (e.g., higher importance)
                (false, false) => 0, // No connection
            };
        }

        matrix
    } 

    pub fn transpose_matrix(&self, matrix: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut transposed = vec![vec![0i8; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = matrix[i][j];
            }
        }
        transposed
    }

    pub fn print_matrix(&self, matrix: &Vec<Vec<i8>>) {
        println!("🔢 Matrix [{}x{}]:", matrix.len(), if matrix.is_empty() { 0 } else { matrix[0].len() });
        for row in matrix {
            let row_str: String = row.iter().map(|&val| val.to_string()).collect::<Vec<String>>().join(" ");
            println!("[ {} ]", row_str);
        }
    }
}