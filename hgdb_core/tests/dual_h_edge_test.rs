use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property};
use hgdb_core::hyper_edge::services::simple_h_edge_service::DualHyperEdgeService;

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, fs::remove_dir_all};

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/dual-h-edge"; // RocksDB path

    #[test]
    fn test_create_dual_h_edge() -> Result<(), Box<dyn Error>> {
        // Delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        // Initialize repository and service
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;
        let service = DualHyperEdgeService::new(&repository);

        // Define test data with tuples of (key, SimpleHyperEdge)
        let edges = vec![
            ("Prime_test_edge_1", SimpleHyperEdge {
                id: "Prime_test_edge_1".to_string(),
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
            ("Prime_test_edge_2", SimpleHyperEdge {
                id: "Prime_test_edge_2".to_string(),
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
            ("Prime_test_edge_3", SimpleHyperEdge {
                id: "Prime_test_edge_3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["not-linked".to_string()]
                    }
                ],
                traversable: true,
                directed: true,
                head_hyper_nodes: Box::new(vec!["v6".to_string()]),
                tail_hyper_nodes: Some(Box::new(vec!["v7".to_string(), "v8".to_string()]))
            })
        ];

        // Create all edges using the tuple key
        for (key, edge) in &edges {
            repository.create(key, edge)?;
        }

        // Ensure the original directed hyperedge is saved (using test_edge_1)
        let retrieved_edge = repository.get_by_key("Prime_test_edge_1")?;
        assert!(retrieved_edge.is_some(), "Edge 'Prime_test_edge_1' was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "e1", "Original edge name mismatch");

        // Validate stored edges
        let all_edges = repository.get_all()?;
        assert_eq!(all_edges.len(), edges.len(), "❌ Not all edges were stored correctly!");

        for (key, edge) in &edges {
            let retrieved_edge = repository.get_by_key(key)?;
            assert!(retrieved_edge.is_some(), "❌ Edge '{}' not found", edge.id);
            assert_eq!(retrieved_edge.as_ref().unwrap().name, edge.name, "❌ Mismatch for edge '{}'", edge.id);

            let dual_id = format!("dual_{}", edge.id);
            service.create_dual_h_edge(&edge.id)?;

            let dual_edge = repository.get_dual_by_key(&dual_id)?;
            assert!(dual_edge.is_some(), "❌ Dual hyperedge '{}' not found", dual_id);
            let dual_edge = dual_edge.unwrap();
            assert_eq!(dual_edge.name, format!("Dual of {}", edge.name), "❌ Dual edge name mismatch");

            let mut test_nodes: Vec<String> = edge.head_hyper_nodes.iter().cloned().collect();
            if let Some(tail_nodes) = &edge.tail_hyper_nodes {
                test_nodes.extend(tail_nodes.iter().cloned());
            }

            let incidence_matrix = service.create_incidence_matrix(&test_nodes, edge);
            let transposed_matrix = service.transpose_matrix(&incidence_matrix);

            // Validate stored matrices
            assert_eq!(dual_edge.incidence_matrix, incidence_matrix, "❌ Incidence matrix mismatch for '{}'", dual_id);
            assert_eq!(dual_edge.transposed_matrix, transposed_matrix, "❌ Transposed matrix mismatch for '{}'", dual_id);

            // Validate matrix dimensions
            let expected_rows = test_nodes.len();
            let expected_cols = 1;
            let expected_transposed_rows = expected_cols;
            let expected_transposed_cols = expected_rows;

            assert_eq!(incidence_matrix.len(), expected_rows, "❌ Incidence matrix rows incorrect");
            assert_eq!(incidence_matrix[0].len(), expected_cols, "❌ Incidence matrix cols incorrect");
            assert_eq!(transposed_matrix.len(), expected_transposed_rows, "❌ Transposed matrix rows incorrect");
            assert_eq!(transposed_matrix[0].len(), expected_transposed_cols, "❌ Transposed matrix cols incorrect");

            // Validate weighted values
            for (i, node) in test_nodes.iter().enumerate() {
                let is_in_head = edge.head_hyper_nodes.contains(node);
                let is_in_tail = edge.tail_hyper_nodes.as_ref().map_or(false, |nodes| nodes.contains(node));
                let expected_weight = match (is_in_head, is_in_tail) {
                    (true, false) => 1,
                    (false, true) => 2,
                    (true, true) => 3,
                    (false, false) => 0,
                };
                assert_eq!(
                    incidence_matrix[i][0], expected_weight,
                    "❌ Incorrect weight at [{}][0] for node '{}' in edge '{}'",
                    i, node, edge.id
                );
            }

            // Validate transposed matrix
            for row in 0..expected_transposed_rows {
                for col in 0..expected_transposed_cols {
                    assert_eq!(
                        transposed_matrix[row][col], incidence_matrix[col][row],
                        "❌ Transposed matrix value mismatch at [{}][{}] in edge '{}'",
                        row, col, edge.id
                    );
                }
            }
        }

        println!("🔍 Listing all stored keys in RocksDB:");
        for item in repository.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((key, _)) => println!("Stored Key: {}", String::from_utf8_lossy(&key)),
                Err(e) => eprintln!("❌ Error iterating DB: {:?}", e),
            }
        }

        Ok(())
    }
}
