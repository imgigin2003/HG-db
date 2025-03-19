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

    pub fn create_dual_h_edge(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let simple_h_edge = self.repository.get_by_key(id)?;
        let original_edge = simple_h_edge.ok_or_else(|| {
            let msg = format!("No SimpleHyperEdge found for key: {}", id);
            eprintln!("❌ {}", msg);
            Box::<dyn Error>::from(msg)
        })?;

        // Extract node IDs from head_hyper_nodes
        let head_nodes = original_edge
            .head_hyper_nodes
            .as_ref()
            .map(|nodes| nodes.iter().map(|node| node.id.clone()).collect::<Vec<String>>())
            .unwrap_or_default();
        let mut nodes_set = head_nodes.clone();

        // Extend with tail_hyper_nodes if present
        if let Some(tail_nodes) = &original_edge.tail_hyper_nodes {
            nodes_set.extend(tail_nodes.iter().map(|node| node.id.clone()));
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
            prime_simple_hyper_edge: std::borrow::Cow::Borrowed(&original_edge),
            dual_properties: original_edge.main_properties.clone(),
            traversable: original_edge.traversable,
            head_hyper_nodes: Box::new(head_nodes),
            tail_hyper_nodes: original_edge
                .tail_hyper_nodes
                .as_ref()
                .map(|nodes| Box::new(nodes.iter().map(|node| node.id.clone()).collect::<Vec<String>>())),
            incidence_matrix,
            transposed_matrix,
        };

        println!("🛠 Saving Dual Hyperedge with Key: {}", dual_edge.id);
        self.repository.save_dual(dual_edge)?;
        println!("✅ Dual Hyperedge saved successfully \n");

        Ok(())
    }

    pub fn create_incidence_matrix<T: ToString>(
        &self,
        nodes: &[T],
        original_edge: &SimpleHyperEdge<String, String, String>,
    ) -> Vec<Vec<i8>> {
        let mut matrix = vec![vec![0i8; 1]; nodes.len()]; // Initialize with 1 column for one edge

        let head_node_ids: Vec<String> = original_edge
            .head_hyper_nodes
            .as_ref()
            .expect("Head nodes should exist")
            .iter()
            .map(|node| node.id.clone())
            .collect();
        let tail_node_ids: Vec<String> = original_edge
            .tail_hyper_nodes
            .as_ref()
            .map_or(vec![], |tail| tail.iter().map(|node| node.id.clone()).collect());

        for (i, node) in nodes.iter().enumerate() {
            let node_str = node.to_string();
            let is_in_head = head_node_ids.contains(&node_str);
            let is_in_tail = tail_node_ids.contains(&node_str);

            // Assign the weight based on head and tail nodes
            matrix[i][0] = match (is_in_head, is_in_tail) {
                (true, false) => 1,  // Head node weight
                (false, true) => 2,  // Tail node weight
                (true, true) => 3,   // Both head and tail node weight
                (false, false) => 0, // No connection
            };
        }

        matrix
    }

    pub fn transpose_matrix(&self, matrix: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
        let mut transposed: Vec<Vec<i8>> = Vec::new();

        // Transpose logic: Convert rows to columns and columns to rows
        for col_idx in 0..matrix[0].len() {
            let mut new_row: Vec<i8> = Vec::new();
            for row in matrix.iter() {
                new_row.push(row[col_idx]);
            }
            transposed.push(new_row);
        }

        transposed
    }

    pub fn print_matrix(&self, matrix: &Vec<Vec<i8>>) {
        println!(
            "🔢 Matrix [{}x{}]:",
            matrix.len(),
            if matrix.is_empty() { 0 } else { matrix[0].len() }
        );
        for row in matrix {
            let row_str: String = row
                .iter()
                .map(|&val| val.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("[ {} ]", row_str);
        }
    }
}