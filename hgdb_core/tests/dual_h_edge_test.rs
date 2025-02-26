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

        // Create all edges using the tuple key
        for (key, edge) in &edges {
            repository.create(key, edge)?;
        }

        // Ensure the original directed hyperedge is saved (using test_edge_1)
        let retrieved_edge = repository.get_by_key("test_edge_1")?;
        assert!(retrieved_edge.is_some(), "Edge 'test_edge_1' was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "e1", "Original edge name mismatch");

        // Visualize all edges based on 'directed' flag using 'name'
        let all_edges = repository.get_all()?;
        let mut output = String::new();
        for edge in &all_edges {
            let head_nodes: Vec<String> = edge.head_hyper_nodes.iter().cloned().collect();
            let head_list = head_nodes.join(", ");
            if edge.directed {
                let tail_nodes: Vec<String> = edge.tail_hyper_nodes.as_ref()
                    .map_or(vec![], |nodes| nodes.iter().cloned().collect());
                let tail_list = tail_nodes.join(", ");
                output.push_str(&format!("{}: head:[{}] -> tail:[{}]", edge.name, head_list, tail_list));
            } else {
                output.push_str(&format!("{}:[{}]", edge.name, head_list));
            }
            if edge.id != all_edges.last().unwrap().id {
                output.push_str(", ");
            }
        }
        println!("✅ Hypergraph: {}", output);

        // Create the dual hyperedge for test_edge_1
        service.create_dual_h_edge("test_edge_1")?;

        let dual_edge = repository.get_dual_by_key(&format!("dual_test_edge_1"))?;
        assert!(dual_edge.is_some(), "❌ Dual hyperedge not found");
        let dual = dual_edge.unwrap();
        println!("✅ Dual Hyperedge: {:?}", dual);
        assert_eq!(dual.name, "Dual of e1", "❌ Dual edge name mismatch"); // Adjusted to match name

        // Define test nodes dynamically
        let test_nodes = vec![
            "v1".to_string(), 
            "v2".to_string(), 
            "v3".to_string(),
        ];

        let incidence_matrix = service.create_incidence_matrix(&test_nodes, &edges[0].1); // Use first edge
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
            let is_in_edge = edges[0].1.head_hyper_nodes.contains(node) || 
                edges[0].1.tail_hyper_nodes.as_ref().map_or(false, |nodes| nodes.contains(node));
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
