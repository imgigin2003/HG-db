use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::repository::Repository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use hgdb_core::db_config::BASE_DB_PATH;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const SUBFOLDER: &str = "simple-hyper-edge-property";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    #[test]
    fn test_default_properties() -> Result<(), Box<dyn Error>> {
        // Clean up test directory
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        let repository = SimpleHyperEdgeRepository::new(DB_PATH.as_str())?;

        // Create test nodes
        let nodes = vec!["v1", "v2", "v3"]
            .into_iter()
            .map(|id| SimpleHyperEdge {
                id: id.to_string(),
                name: id.to_string(),
                main_properties: vec![],
                traversable: false,
                directed: false,
                head_hyper_nodes: None,
                tail_hyper_nodes: None,
                incidence_matrix: vec![]
            })
            .collect::<Vec<_>>();

        let edge = SimpleHyperEdge {
            id: "test_edge_1".to_string(),
            name: "e1".to_string(),
            main_properties: vec![],
            traversable: true,
            directed: true,
            head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
            tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone()])),
            incidence_matrix: vec![]
        };

        repository.create("test_edge", edge)?;
        let retrieved = repository.get_by_key("test_edge")?.unwrap();
        
        // Verify default properties were added
        assert!(retrieved.main_properties.iter().any(|p| p.key == "layer"));
        assert!(retrieved.main_properties.iter().any(|p| p.key == "level"));
        assert!(retrieved.main_properties.iter().any(|p| p.key == "aspect"));
        
        // Verify default values
        assert_eq!(
            retrieved.main_properties
                .iter()
                .find(|p| p.key == "layer")
                .unwrap()
                .value[0],
            "0"
        );
        assert_eq!(
            retrieved.main_properties
                .iter()
                .find(|p| p.key == "level")
                .unwrap()
                .value[0],
            "0"
        );
        assert_eq!(
            retrieved.main_properties
                .iter()
                .find(|p| p.key == "aspect")
                .unwrap()
                .value[0],
            "0"
        );
        Ok(())
    } 
}