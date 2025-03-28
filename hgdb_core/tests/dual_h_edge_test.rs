use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use hgdb_core::hyper_edge::services::simple_h_edge_service::DualHyperEdgeService;
<<<<<<< HEAD
use hgdb_core::db_config::BASE_DB_PATH;
=======
use hgdb_core::hyper_edge::repository::*;
>>>>>>> parent of 6c9614f (Revert "Fixed methods")

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, fs::remove_dir_all};

    const SUBFOLDER: &str = "dual_hyper-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    #[test]
    fn test_dual_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        let repository = SimpleHyperEdgeRepository::new(DB_PATH.as_str())?;
        let service = DualHyperEdgeService::new(&repository);

        // Define nodes as SimpleHyperEdge instances
        let nodes = vec![
            ("v1", "v1"), ("v2", "v2"), ("v3", "v3"), ("v4", "v4"),
            ("v5", "v5"), ("v6", "v6"), ("v7", "v7"), ("v8", "v8"),
        ].into_iter().map(|(id, name)| SimpleHyperEdge {
            id: id.to_string(),
            name: name.to_string(),
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
        }).collect::<Vec<_>>();

        let edges = vec![
            ("Prime_test_edge_1", SimpleHyperEdge {
                id: "Prime_test_edge_1".to_string(),
                name: "e1".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone()])),
            }),
            ("Prime_test_edge_2", SimpleHyperEdge {
                id: "Prime_test_edge_2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["not-linked".to_string()],
                    p_type: PropertyType::Structure,
                }],
                traversable: false,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[3].clone(), nodes[4].clone()])),
                tail_hyper_nodes: None,
            }),
            ("Prime_test_edge_3", SimpleHyperEdge {
                id: "Prime_test_edge_3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["not-linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![nodes[5].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[6].clone(), nodes[7].clone()])),
            }),
        ];

        for (key, edge) in &edges {
            repository.create(key, edge.clone())?;
        }

        let retrieved_edge = repository.get_by_key("Prime_test_edge_1")?;
        assert!(retrieved_edge.is_some(), "Edge 'Prime_test_edge_1' was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "e1", "Original edge name mismatch");

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

            let mut test_nodes: Vec<String> = edge.head_hyper_nodes.as_ref()
                .map_or(vec![], |nodes| nodes.iter().map(|n| n.id.clone()).collect());
            if let Some(tail_nodes) = &edge.tail_hyper_nodes {
                test_nodes.extend(tail_nodes.iter().map(|n| n.id.clone()));
            }

            let incidence_matrix = service.create_incidence_matrix(&test_nodes, edge);
            let transposed_matrix = service.transpose_matrix(&incidence_matrix);

            assert_eq!(dual_edge.incidence_matrix, incidence_matrix, "❌ Incidence matrix mismatch for '{}'", dual_id);
            assert_eq!(dual_edge.transposed_matrix, transposed_matrix, "❌ Transposed matrix mismatch for '{}'", dual_id);

            let expected_rows = test_nodes.len();
            let expected_cols = 1;
            let expected_transposed_rows = expected_cols;
            let expected_transposed_cols = expected_rows;

            assert_eq!(incidence_matrix.len(), expected_rows, "❌ Incidence matrix rows incorrect");
            assert_eq!(incidence_matrix[0].len(), expected_cols, "❌ Incidence matrix cols incorrect");
            assert_eq!(transposed_matrix.len(), expected_transposed_rows, "❌ Transposed matrix rows incorrect");
            assert_eq!(transposed_matrix[0].len(), expected_transposed_cols, "❌ Transposed matrix cols incorrect");

            for (i, node) in test_nodes.iter().enumerate() {
                let is_in_head = edge.head_hyper_nodes.as_ref()
                    .map_or(false, |nodes| nodes.iter().any(|n| n.id == *node));
                let is_in_tail = edge.tail_hyper_nodes.as_ref()
                    .map_or(false, |nodes| nodes.iter().any(|n| n.id == *node));
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