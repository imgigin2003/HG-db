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
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        let repository = SimpleHyperEdgeRepository::new(DB_PATH.as_str())?;

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
            incidence_matrix: vec![],
        }).collect();

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
                head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone(), nodes[2].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[3].clone(), nodes[4].clone()])),
                incidence_matrix: vec![],
            }),
            ("e2", SimpleHyperEdge {
                id: "e2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![],
                traversable: false,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[5].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[6].clone()])),
                incidence_matrix: vec![],
            }),
        ];

        let hypergraph_2 = vec![
            ("e3", SimpleHyperEdge {
                id: "e3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![],
                traversable: true,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![nodes[0].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[1].clone(), nodes[2].clone()])),
                incidence_matrix: vec![],
            }),
        ];

        let hypergraph_3 = vec![
            ("e4", SimpleHyperEdge {
                id: "e4".to_string(),
                name: "e4".to_string(),
                main_properties: vec![],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[3].clone(), nodes[4].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[5].clone()])),
                incidence_matrix: vec![],
            }),
        ];

        let mut all_edges = std::collections::HashMap::new();
        all_edges.extend(hypergraph_1.clone());
        all_edges.extend(hypergraph_2.clone());
        all_edges.extend(hypergraph_3.clone());

        // Create edges
        for (key, edge) in &all_edges {
            repository.create(key, edge.clone())?;
        }

        let stored_edges = repository.get_all()?;
        assert_eq!(stored_edges.len(), all_edges.len(), "Not all edges were stored correctly");

        // Check incidence matrix consistency for every stored edge
        for (key, original_edge) in &all_edges {
            let expected_matrix = repository.create_incidence_matrix(original_edge);

            let stored_edge = repository.get_by_key(key)?
                .ok_or_else(|| format!("Edge {} not found after creation", key))?;

            assert_eq!(
                stored_edge.incidence_matrix,
                expected_matrix,
                "Incidence matrix mismatch for edge '{}'", key
            );
        }

        // JSON export part
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

        // Cleanup
        for (key, _) in &all_edges {
            repository.delete(key)?;
        }

        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "Database should be empty after deleting all edges");

        Ok(())
    }
}