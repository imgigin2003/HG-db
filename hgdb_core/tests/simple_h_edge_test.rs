use hgdb_core::db_config::BASE_DB_PATH;
use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use serde_json::to_string_pretty;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::{remove_dir_all, File, metadata};
    use std::io::Write;
    use serde_json::json;

    const SUBFOLDER: &str = "simple-h-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
        static ref JSON_PATH: String = format!("{}/test_hypergraphs.json", DB_PATH.as_str());
    }

    #[test]
    fn test_simple_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        // Initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH.as_str())?;

        // Define all nodes in a Vec
        let nodes: Vec<SimpleHyperEdge<String, String, String>> = vec![
            "v1", "v2", "v3", "v4", "v5", "v6", "v7"
        ].into_iter().map(|id| SimpleHyperEdge {
            id: id.to_string(),
            name: id.to_string(),
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
        }).collect();

        // Define edges for three hypergraphs
        let hypergraph_1 = vec![
            ("e1", SimpleHyperEdge {
                id: "e1".to_string(),
                name: "e1".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![
                    nodes[0].clone(), // v1
                    nodes[1].clone(), // v2
                    nodes[2].clone(), // v3
                ])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone()])), // v3
            }),
            ("e2", SimpleHyperEdge {
                id: "e2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![
                    nodes[0].clone(), // v1
                    nodes[1].clone(), // v2
                    nodes[6].clone(), // v7
                ])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[6].clone()])), // v7
            }),
        ];

        let hypergraph_2 = vec![
            ("e3", SimpleHyperEdge {
                id: "e3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![
                    nodes[5].clone(), // v6
                    nodes[6].clone(), // v7
                ])),
                tail_hyper_nodes: None,
            }),
            ("e6", SimpleHyperEdge {
                id: "e6".to_string(),
                name: "e6".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![nodes[2].clone()])), // v3
                tail_hyper_nodes: Some(Box::new(vec![nodes[3].clone()])), // v4
            }),
            ("e7", SimpleHyperEdge {
                id: "e7".to_string(),
                name: "e7".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![
                    nodes[3].clone(), // v4
                    nodes[6].clone(), // v7
                ])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[3].clone()])), // v4
            }),
        ];

        let hypergraph_3 = vec![
            ("e4", SimpleHyperEdge {
                id: "e4".to_string(),
                name: "e4".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[4].clone()])), // v5
                tail_hyper_nodes: None,
            }),
            ("e5", SimpleHyperEdge {
                id: "e5".to_string(),
                name: "e5".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[3].clone()])), // v4
                tail_hyper_nodes: None,
            }),
        ];

        // Combine all edges for repository operations
        let all_edges: Vec<_> = hypergraph_1.iter().chain(hypergraph_2.iter()).chain(hypergraph_3.iter()).collect();

        // Create edges
        for (key, edge) in &all_edges {
            match repository.create(key, edge) {  // Clone edge here to avoid borrowing issues
                Ok(_) => println!("✅ Successfully created edge: {}", key),
                Err(e) => eprintln!("❌ Failed to create edge {}: {:?}", key, e),
            }
        }

        // Retrieve all edges and verify count
        let stored_edges = repository.get_all()?;
        assert_eq!(stored_edges.len(), all_edges.len(), "❌ Not all edges were stored correctly");

        // Structure the JSON with labels for each hypergraph
        let json_structure = json!({
            "hypergraph1": hypergraph_1.iter().map(|(_, edge)| edge).collect::<Vec<_>>(),
            "hypergraph2": hypergraph_2.iter().map(|(_, edge)| edge).collect::<Vec<_>>(),
            "hypergraph3": hypergraph_3.iter().map(|(_, edge)| edge).collect::<Vec<_>>()
        });

        // Serialize to a pretty-printed JSON string and write to DB_PATH
        let json_data = to_string_pretty(&json_structure)?;
        let mut file = File::create(JSON_PATH.as_str())?;
        file.write_all(json_data.as_bytes())?;
        assert!(metadata(JSON_PATH.as_str())?.is_file(), "❌ JSON file was not created at expected path");

        // Clean up: Delete all edges
        for (key, _) in &all_edges {
            repository.delete(key)?;
        }

        // Verify database is empty
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "❌ Database should be empty after deleting all edges");

        Ok(())
    }
}