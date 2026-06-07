use hgdb_core::hyper_edge::repository::h_edge_repository::HyperEdgeRepository;
use hgdb_core::hyper_edge::repository::Repository;
use hgdb_core::hyper_edge::entity::h_edge::hyper_edge::HyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge;
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;
use hgdb_core::db_config::BASE_DB_PATH;
use std::error::Error;
use std::fs::{File, remove_dir_all};
use std::io::{Write, Read};

#[cfg(test)]
mod tests {
    use super::*;

    const SUBFOLDER: &str = "hyper-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    #[test]
    fn test_hyper_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove DB directory '{}': {:?}", DB_PATH.as_str(), e).into());
            }
        }

        let repository = HyperEdgeRepository::new(DB_PATH.as_str())?;

        let log_file_path = format!("{}/test.log", DB_PATH.as_str());

        let mut file = File::create(&log_file_path)?;
        file.write_all(b"Test log content")?;

        let test_edge = HyperEdge {
            id: "test_edge_1".to_string(),
            light_hyper_edge: LightHyperEdge {
                id: "test_edge_1".to_string(),
                prime_simple_hyper_edge: SimpleHyperEdge {
                    id: "test_edge_1".to_string(),
                    name: "test_edge_1".to_string(),
                    main_properties: vec![],
                    traversable: true,
                    directed: false,
                    head_hyper_nodes: Some(vec!["v1".to_string(), "v2".to_string()]),
                    tail_hyper_nodes: Some(vec!["v3".to_string(), "v4".to_string()]),
                    incidence_matrix: vec![]
                },
                structural_properties: vec![StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
                    layer_index: None,
                }],
                relationship: Relationship {
                    node_1: "v2".to_string(),
                    node_2: "v3".to_string(),
                    directed: true,
                    edge_properties: vec!["weight: 10".to_string(), "type: strong".to_string()],
                },
                traverse: Traverse {
                    path: vec!["v2".to_string(), "v3".to_string(), "v4".to_string()],
                },
            },
            attachments: vec![log_file_path.clone()],
        };

        repository.create("test_edge_1", test_edge)?;

        let updated_edge = HyperEdge {
            id: "test_edge_1".to_string(),
            light_hyper_edge: LightHyperEdge {
                id: "test_edge_1".to_string(),
                prime_simple_hyper_edge: SimpleHyperEdge {
                    id: "test_edge_1".to_string(),
                    name: "UpdatedConnection".to_string(),
                    main_properties: vec![],
                    traversable: true,
                    directed: true,
                    head_hyper_nodes: Some(vec!["v1".to_string(), "v2".to_string()]),
                    tail_hyper_nodes: Some(vec!["v3".to_string(), "v4".to_string()]),
                    incidence_matrix: vec![]
                },
                structural_properties: vec![StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
                    layer_index: None,
                }],
                relationship: Relationship {
                    node_1: "v2".to_string(),
                    node_2: "v3".to_string(),
                    directed: true,
                    edge_properties: vec!["weight: 10".to_string(), "type: strong".to_string()],
                },
                traverse: Traverse {
                    path: vec!["v2".to_string(), "v3".to_string(), "v4".to_string()],
                },
            },
            attachments: vec![log_file_path.clone()],
        };

        repository.update("test_edge_1", &updated_edge)?;
        println!("Successfully updated edge: test_edge_1");

        let updated_edge = repository.get_by_key("test_edge_1")?.expect("Updated edge should exist");
        let mut updated_files = repository.open_attachments(&updated_edge)?;
        assert_eq!(updated_files.len(), 1, "Failed to open the log file attachment from updated edge");

        let mut updated_buffer = Vec::new();
        updated_files[0].read_to_end(&mut updated_buffer)?;
        println!("Updated log file size: {} bytes", updated_buffer.len());
        assert!(!updated_buffer.is_empty(), "Updated log file appears to be empty");

        assert_eq!(
            updated_edge.light_hyper_edge.prime_simple_hyper_edge.name,
            "UpdatedConnection",
            "Name property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_edge.prime_simple_hyper_edge.directed,
            true,
            "Directed property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_edge.relationship.edge_properties,
            vec!["weight: 10".to_string(), "type: strong".to_string()],
            "Edge properties not updated"
        );

        repository.delete("test_edge_1")?;
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "Database should be empty after deletion");

        Ok(())
    }
}