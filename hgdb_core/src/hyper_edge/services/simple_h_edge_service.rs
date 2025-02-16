use crate::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use crate::hyper_edge::entity::dual_h_edge::DualHyperEdge;
use std::error::Error;

pub struct DualHyperEdgeService {
    repository: SimpleHyperEdgeRepository
}

impl DualHyperEdgeService {
    pub fn new(repository: SimpleHyperEdgeRepository) -> Self {
        DualHyperEdgeService { repository }
    }

    //create a dual graph based on an existing simple hypergraph
    pub fn create_dual_h_edge(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let simple_h_edge = self.repository.get_by_key(id)?;

        if let Some(original_edge) = simple_h_edge {
            let dual_edge = DualHyperEdge {
                id: format!("dual_{}", original_edge.id),
                simple_hyper_edge: original_edge.clone(),
                dual_properties: original_edge.main_properties.clone(),
            };

            self.repository.save_dual(dual_edge)?;

            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Hyperedge not found")))
        }
    }
}