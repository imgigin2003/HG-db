use hgdb_core::hyper_edge::repository::h_edge_repository::HyperEdgeRepository;
use hgdb_core::hyper_edge::repository::Repository;
use hgdb_core::hyper_edge::entity::h_edge::hyper_edge::HyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;
use hgdb_core::db_config::BASE_DB_PATH;
use std::error::Error;
use std::fs::{File, remove_dir_all};
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const SUBFOLDER: &str = "hyper-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    #[test]
    fn test_hyper_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Clean up database directory before starting
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove DB directory '{}': {:?}", DB_PATH.as_str(), e).into());
            }
        }

        let repository = HyperEdgeRepository::new(DB_PATH.as_str())?;
        // Create a dummy log file
        let log_file_path = format!("{}/test.log", DB_PATH.as_str());
        let mut file = File::create(&log_file_path)?;
        file.write_all(b"Test log content")?;
        let log_file_path = std::path::PathBuf::from(log_file_path);

        println!("Using log file: {}", log_file_path.display());

        let nodes = vec!["v1", "v2", "v3", "v4"]
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

        let edges = vec![HyperEdge {
            id: "test_edge_1".to_string(),
            light_hyper_edge: LightHyperEdge {
                id: "e1".to_string(),
                prime_simple_hyper_edge: SimpleHyperEdge {
                    id: "e1".to_string(),
                    name: "test_edge_1".to_string(),
                    main_properties: vec![Property {
                        key: "type".to_string(),
                        value: vec!["linked".to_string()],
                        p_type: PropertyType::Simple,
                    }],
                    traversable: true,
                    directed: false,
                    head_hyper_nodes: Some(Box::new(vec![nodes[0].clone()])),
                    tail_hyper_nodes: None,
                    incidence_matrix: vec![]
                },
                structural_properties: vec![StructuralProperty {
                    address: vec!["123 Main St".to_string(), "Apt 4B".to_string()],
                }],
                relationship: Relationship {
                    node_1: "v1".to_string(),
                    node_2: "v2".to_string(),
                    directed: true,
                    edge_properties: vec!["weight: 5".to_string(), "type: weak".to_string()],
                },
                traverse: Traverse {
                    path: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()],
                },
            },
            attachments: vec![log_file_path.clone()],
        }];

        for edge in &edges {
            repository.create(&edge.id, edge.clone())?;
            println!("✅ Successfully created edge: {}", edge.id);
        }

        let retrieved_edge = repository.get_by_key("test_edge_1")?.expect("❌ Initial edge should exist");
        let mut files = repository.open_attachments(&retrieved_edge)?;
        assert_eq!(files.len(), 1, "❌ Failed to open the log file attachment from initial edge");

        let mut buffer = Vec::new();
        files[0].read_to_end(&mut buffer)?;
        println!("Initial log file size: {} bytes", buffer.len());
        assert!(!buffer.is_empty(), "❌ Initial log file appears to be empty");

        let updated_edge = HyperEdge {
            id: "test_edge_1".to_string(),
            light_hyper_edge: LightHyperEdge {
                id: "e1".to_string(),
                prime_simple_hyper_edge: SimpleHyperEdge {
                    id: "e1".to_string(),
                    name: "UpdatedConnection".to_string(),
                    main_properties: vec![Property {
                        key: "type".to_string(),
                        value: vec!["strongly linked".to_string()],
                        p_type: PropertyType::Simple,
                    }],
                    traversable: false,
                    directed: true,
                    head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
                    tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone(), nodes[3].clone()])),
                    incidence_matrix: vec![]
                },
                structural_properties: vec![StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
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
        println!("✅ Successfully updated edge: test_edge_1");

        let updated_edge = repository.get_by_key("test_edge_1")?.expect("❌ Updated edge should exist");
        let mut updated_files = repository.open_attachments(&updated_edge)?;
        assert_eq!(updated_files.len(), 1, "❌ Failed to open the log file attachment from updated edge");

        let mut updated_buffer = Vec::new();
        updated_files[0].read_to_end(&mut updated_buffer)?;
        println!("Updated log file size: {} bytes", updated_buffer.len());
        assert!(!updated_buffer.is_empty(), "❌ Updated log file appears to be empty");

        assert_eq!(
            updated_edge.light_hyper_edge.prime_simple_hyper_edge.name,
            "UpdatedConnection",
            "❌ Name property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_edge.prime_simple_hyper_edge.directed,
            true,
            "❌ Directed property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_edge.relationship.edge_properties,
            vec!["weight: 10".to_string(), "type: strong".to_string()],
            "❌ Edge properties not updated"
        );

        repository.delete("test_edge_1")?;
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "❌ Database should be empty after deletion");

        Ok(())
    }
}