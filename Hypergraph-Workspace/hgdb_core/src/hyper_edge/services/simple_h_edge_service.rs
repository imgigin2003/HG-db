use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge;
use crate::hyper_edge::repository::*;
use std::error::Error;

pub struct DualHyperEdgeService<'a, R>
where
    R: Repository<SimpleHyperEdge<String, String, String>>,
{
    repository: &'a R,
}

impl<'a, R> DualHyperEdgeService<'a, R>
where
    R: Repository<SimpleHyperEdge<String, String, String>>,
{
    pub fn new(repository: &'a R) -> Self {
        DualHyperEdgeService { repository }
    }

    pub fn create_dual_h_edge(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let simple_h_edge = self.repository.get_by_key(id)?;
        let original_edge = simple_h_edge.ok_or_else(|| {
            let msg = format!("No SimpleHyperEdge found for key: {}", id);
            Box::<dyn Error>::from(msg)
        })?;

        let head_nodes = original_edge.head_hyper_nodes.as_ref().map_or_else(
            || Vec::new(),
            |ids| ids.clone()
        );
        let mut nodes_set = head_nodes.clone();

        if let Some(tail_nodes) = &original_edge.tail_hyper_nodes {
            nodes_set.extend(tail_nodes.clone());
        }

        let incidence_matrix = self.create_incidence_matrix(&nodes_set, &original_edge);
        let transposed_matrix = self.transpose_matrix(&incidence_matrix);

        let dual_edge_id = format!("dual_{}", id);
        let dual_edge = DualHyperEdge {
            id: dual_edge_id.clone(),
            name: format!("Dual of {}", original_edge.name),
            prime_simple_hyper_edge: original_edge.clone(),
            dual_properties: original_edge.main_properties.clone(),
            traversable: original_edge.traversable,
            head_hyper_nodes: head_nodes,
            tail_hyper_nodes: original_edge.tail_hyper_nodes.clone().unwrap_or_default(),
            incidence_matrix,
            transposed_matrix,
        };

        self.repository.save_dual(dual_edge)?;
        Ok(())
    }

    pub fn create_incidence_matrix(
        &self,
        nodes: &[String],
        original_edge: &SimpleHyperEdge<String, String, String>,
    ) -> Vec<Vec<i8>> {
        let mut matrix = vec![vec![0i8; 1]; nodes.len()];
        let head_node_ids: Vec<String> = original_edge
            .head_hyper_nodes
            .as_ref()
            .map_or_else(|| Vec::new(), |ids| ids.clone());
        let tail_node_ids: Vec<String> = original_edge
            .tail_hyper_nodes
            .as_ref()
            .map_or_else(|| Vec::new(), |ids| ids.clone());

        for (i, node) in nodes.iter().enumerate() {
            let is_in_head = head_node_ids.contains(node);
            let is_in_tail = tail_node_ids.contains(node);
            matrix[i][0] = match (is_in_head, is_in_tail) {
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
                (false, false) => 0,
            };
        }
        matrix
    }

    pub fn transpose_matrix(&self, matrix: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
        if matrix.is_empty() {
            return Vec::new();
        }
        
        let mut transposed: Vec<Vec<i8>> = Vec::new();
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
            "Matrix [{}x{}]:",
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