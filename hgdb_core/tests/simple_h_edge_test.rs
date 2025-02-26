use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge"; // RocksDB path

    #[test]
    fn test_simple_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        // Initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;

        // Define test edges with unique keys
        let edges = vec![
            ("test_edge_1", SimpleHyperEdge {
                id: "test_edge_1".to_string(),
                name: "e1".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["linked".to_string()]
                    }
                ],
                traversable: true,
                directed: true,
                head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
                tail_hyper_nodes: Some(Box::new(vec!["v3".to_string()]))
            }),
            ("test_edge_2", SimpleHyperEdge {
                id: "test_edge_2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["not-linked".to_string()]
                    }
                ],
                traversable: false,
                directed: false,
                head_hyper_nodes: Box::new(vec!["v4".to_string(), "v5".to_string()]),
                tail_hyper_nodes: None
            }),
            ("test_edge_3", SimpleHyperEdge {
                id: "test_edge_3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["not-linked".to_string()]
                    }
                ],
                traversable: true,
                directed: false,
                head_hyper_nodes: Box::new(vec!["v6".to_string(), "v7".to_string(), "v8".to_string()]),
                tail_hyper_nodes: None
            })
        ];

        // Create edges
        for (key, edge) in &edges {
            repository.create(key, edge)?;
        }

        // Retrieve all edges and verify count
        let all_edges = repository.get_all()?;
        assert_eq!(all_edges.len(), edges.len(), "❌ Not all edges were stored correctly");

        // Retrieve and validate each edge
        for (key, original_edge) in &edges {
            let retrieved_edge = repository.get_by_key(key)?;
            assert!(retrieved_edge.is_some(), "❌ Edge {} was not found", key);
            let retrieved_edge = retrieved_edge.unwrap();
            assert_eq!(retrieved_edge.name, original_edge.name, "❌ Retrieved edge name mismatch");
        }

        // Delete all edges
        for (key, _) in &edges {
            repository.delete(key)?;
        }

        // Verify database is empty
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "❌ Database should be empty after deleting all edges");

        Ok(())
    }
}
