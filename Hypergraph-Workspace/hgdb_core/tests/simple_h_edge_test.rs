use hgdb_core::db_config::BASE_DB_PATH;
use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::repository::Repository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use serde_json::{to_string_pretty, json};
use std::fs::{File, metadata};
use std::io::Write;

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, fs::remove_dir_all};

    const SUBFOLDER: &str = "simple-h-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
        static ref JSON_PATH: String = format!("{}/test_hypergraphs.json", DB_PATH.as_str());
    }

    #[test]
    fn test_simple_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("Failed to remove DB directory: {:?}", e);
            }
        }

        let repository = SimpleHyperEdgeRepository::new(DB_PATH.as_str())?;

        let hypergraph_1 = vec![
            ("e1", SimpleHyperEdge {
                id: "e1".to_string(),
                name: "e1".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(vec![
                    "v1".to_string(),
                    "v2".to_string(),
                    "v3".to_string(),
                ]),
                tail_hyper_nodes: Some(vec!["v3".to_string()]),
                incidence_matrix: vec![],
            }),
            ("e2", SimpleHyperEdge {
                id: "e2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(vec![
                    "v1".to_string(),
                    "v2".to_string(),
                    "v7".to_string(),
                ]),
                tail_hyper_nodes: Some(vec!["v7".to_string()]),
                incidence_matrix: vec![],
            }),
        ];

        let hypergraph_2 = vec![
            ("e3", SimpleHyperEdge {
                id: "e3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(vec![
                    "v6".to_string(),
                    "v7".to_string(),
                ]),
                tail_hyper_nodes: None,
                incidence_matrix: vec![],
            }),
            ("e6", SimpleHyperEdge {
                id: "e6".to_string(),
                name: "e6".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(vec!["v3".to_string()]),
                tail_hyper_nodes: Some(vec!["v4".to_string()]),
                incidence_matrix: vec![],
            }),
            ("e7", SimpleHyperEdge {
                id: "e7".to_string(),
                name: "e7".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(vec![
                    "v4".to_string(),
                    "v7".to_string(),
                ]),
                tail_hyper_nodes: Some(vec!["v4".to_string()]),
                incidence_matrix: vec![],
            }),
        ];

        let hypergraph_3 = vec![
            ("e4", SimpleHyperEdge {
                id: "e4".to_string(),
                name: "e4".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(vec!["v5".to_string()]),
                tail_hyper_nodes: None,
                incidence_matrix: vec![],
            }),
            ("e5", SimpleHyperEdge {
                id: "e5".to_string(),
                name: "e5".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(vec!["v4".to_string()]),
                tail_hyper_nodes: None,
                incidence_matrix: vec![],
            }),
        ];

        let all_edges: Vec<_> = hypergraph_1.iter().chain(hypergraph_2.iter()).chain(hypergraph_3.iter()).collect();

        for (key, edge) in &all_edges {
            match repository.create(key, edge.clone()) {
                Ok(_) => println!("Successfully created edge: {}", key),
                Err(e) => eprintln!("Failed to create edge {}: {:?}", key, e),
            }
            repository.save(edge.clone())?;
        }

        let stored_edges = repository.get_all()?;
        assert_eq!(stored_edges.len(), all_edges.len(), "Not all edges were stored correctly");
        
        if !stored_edges.is_empty() {
            assert_eq!(stored_edges[0].incidence_matrix, vec![vec![1], vec![1], vec![3]]);
        }

        for (key, edge) in &all_edges {
            let incidence_matrix = repository.create_incidence_matrix(edge);
            repository.print_matrix(&incidence_matrix, &format!("Incidence Matrix for {}", key));
            
            let retrieved_edge = repository.get_by_key(key)?.unwrap();
            assert_eq!(
                retrieved_edge.incidence_matrix, incidence_matrix,
                "Incidence matrix mismatch for edge '{}'", key
            );
        }

        let json_structure = json!({
            "hypergraph1": hypergraph_1.iter().map(|(_, edge)| {
                let mut edge_with_matrix = edge.clone();
                edge_with_matrix.incidence_matrix = repository.create_incidence_matrix(edge);
                edge_with_matrix
            }).collect::<Vec<_>>(),
            "hypergraph2": hypergraph_2.iter().map(|(_, edge)| {
                let mut edge_with_matrix = edge.clone();
                edge_with_matrix.incidence_matrix = repository.create_incidence_matrix(edge);
                edge_with_matrix
            }).collect::<Vec<_>>(),
            "hypergraph3": hypergraph_3.iter().map(|(_, edge)| {
                let mut edge_with_matrix = edge.clone();
                edge_with_matrix.incidence_matrix = repository.create_incidence_matrix(edge);
                edge_with_matrix
            }).collect::<Vec<_>>()
        });

        let json_data = to_string_pretty(&json_structure)?;
        let mut file = File::create(JSON_PATH.as_str())?;
        file.write_all(json_data.as_bytes())?;
        assert!(metadata(JSON_PATH.as_str())?.is_file(), "JSON file was not created at expected path");

        for (key, _) in &all_edges {
            repository.delete(key)?;
        }

        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "Database should be empty after deleting all edges");

        Ok(())
    }
}