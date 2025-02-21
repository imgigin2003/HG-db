use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};
use hgdb_core::hyper_edge::services::simple_h_edge_service::DualHyperEdgeService;

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, fs::remove_dir_all};

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/dual-h-edge"; // RocksDB path

    #[test]
    fn test_create_dual_h_edge() -> Result<(), Box<dyn Error>> {
        let _ = remove_dir_all(DB_PATH);

        // Initialize repository and service
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;
        let service = DualHyperEdgeService::new(&repository);

        let test_key = "e1";
        let test_edge = SimpleHyperEdge {
            id: test_key.to_string(),
            name: "Friendship".to_string(),
            main_properties: vec![
                Property {
                    key: "relationship-type".to_string(),
                    value: vec!["friends".to_string()],
                }
            ],
            traversable: true,
            directed: true,
            head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
            tail_hyper_nodes: Some(Box::new(vec!["v3".to_string()])),
        };
        repository.create(test_key, &test_edge)?;

        // Ensure the original hyperedge is saved
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "Friendship", "Original edge name mismatch");

        // Create the dual hyperedge
        service.create_dual_h_edge(test_key)?;

        let dual_edge = repository.get_dual_by_key(&format!("dual_{}", test_key))?;
        match dual_edge {
            Some(_) => println!("Dual Hyperedge found✅"),
            None => println!("❌ Failed to find Dual Hyperedge with the expected key"),
        }

        // Define test nodes dynamically
        let test_nodes = vec![
            "v1".to_string(), 
            "v2".to_string(), 
            "v3".to_string(),
        ];

        let incidence_matrix = service.create_incidence_matrix(&test_nodes, &test_edge);
        let transposed_matrix = service.transpose_matrix(&incidence_matrix);

        // Dynamic Assertions Based on Input Size
        let expected_rows = test_nodes.len(); // Rows should match the number of nodes
        let expected_cols = 1; // Each node maps to a single column initially
        let expected_transposed_rows = expected_cols; 
        let expected_transposed_cols = expected_rows; // Transpose swaps rows/cols

        assert_eq!(
            incidence_matrix.len(),
            expected_rows,
            "❌ Incidence matrix row count incorrect"
        );
        assert_eq!(
            incidence_matrix[0].len(),
            expected_cols,
            "❌ Incidence matrix column count incorrect"
        );
        assert_eq!(
            transposed_matrix.len(),
            expected_transposed_rows,
            "❌ Transposed matrix row count incorrect"
        );
        assert_eq!(
            transposed_matrix[0].len(),
            expected_transposed_cols,
            "❌ Transposed matrix column count incorrect"
        );

        // Check if matrix contains correct boolean values
        for (i, node) in test_nodes.iter().enumerate() {
            let is_in_edge = test_edge.head_hyper_nodes.contains(node) || test_edge.tail_hyper_nodes.clone().expect("REASON").contains(node);
            
            assert_eq!(
                incidence_matrix[i][0],
                is_in_edge,
                "❌ Incorrect value at incidence_matrix[{}][0] for node '{}'",
                i, node
            );
        }

        // Validate transposed matrix logic
        for row in 0..expected_transposed_rows {
            for col in 0..expected_transposed_cols {
                assert_eq!(
                    transposed_matrix[row][col],
                    incidence_matrix[col][row],
                    "❌ Transposed matrix value mismatch at [{}][{}]",
                    row, col
                );
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
