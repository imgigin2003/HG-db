use hgdb_core::hyper_edge::repository::h_edge_repository::HyperEdgeRepository; // Import the HyperEdgeRepository struct
use hgdb_core::hyper_edge::entity::h_edge::hyper_edge::HyperEdge; // Import the HyperEdge struct
use hgdb_core::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge; // Import the LightHyperEdge struct
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType}; // Import the SimpleHyperEdge, Property, and PropertyType structs
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse}; // Import the StructuralProperty and Traverse structs
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship; // Import the Relationship struct
use std::error::Error; // Import the Error trait
use std::fs::{self, File};

const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/hyper-edge"; // RocksDB path

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_hyper_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Initialize repository first to ensure RocksDB creates a log file
        let repository = HyperEdgeRepository::new(DB_PATH)?;

        // Find a RocksDB-generated log file in DB_PATH
        let log_file_path = fs::read_dir(DB_PATH)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| {
                path.is_file() && 
                path.extension().map_or(false, |ext| ext == "log")
            })
            .ok_or_else(|| format!("❌ No .log file found in {}", DB_PATH))?;

        println!("Using log file: {}", log_file_path.display());

        // Create initial hyper edge with the log file as an attachment
        let edges = vec![
            HyperEdge {
                id: "test_edge_1".to_string(),
                light_hyper_node: LightHyperEdge {
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
                        head_hyper_nodes: Box::new(vec!["v1".to_string()]),
                        tail_hyper_nodes: None,
                    },
                    structural_properties: vec![
                        StructuralProperty {
                            address: vec!["123 Main St".to_string(), "Apt 4B".to_string()],
                        },
                    ],
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
            },
        ];

        // Create initial edge
        for edge in &edges {
            repository.create(&edge.id, edge)?;
            println!("✅ Successfully created edge: {}", edge.id);
        }

        // Test opening the log file with File::open from initial edge
        let (retrieved_edge_opt, _) = repository.get_by_key("test_edge_1")?;
        let retrieved_edge = retrieved_edge_opt.expect("❌ Initial edge should exist");
        
        let mut files: Vec<File> = retrieved_edge.attachments
            .iter()
            .map(|path| File::open(path))
            .collect::<std::io::Result<_>>()?;
        assert_eq!(
            files.len(),
            1,
            "❌ Failed to open the log file attachment from initial edge"
        );

        // Read as bytes instead of string to avoid UTF-8 validation
        let mut buffer = Vec::new();
        files[0].read_to_end(&mut buffer)?;
        println!("Initial log file size: {} bytes", buffer.len());
        assert!(!buffer.is_empty(), "❌ Initial log file appears to be empty");

        // Create updated hyper edge with the same log file but different properties
        let updated_edge = HyperEdge {
            id: "test_edge_1".to_string(),
            light_hyper_node: LightHyperEdge {
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
                    head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
                    tail_hyper_nodes: Some(Box::new(vec!["v3".to_string(), "v4".to_string()])),
                },
                structural_properties: vec![
                    StructuralProperty {
                        address: vec!["789 Oak St".to_string()],
                    },
                ],
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

        // Update edge
        repository.update("test_edge_1", &updated_edge)?;
        println!("✅ Successfully updated edge: test_edge_1");

        // Test opening the log file with File::open from updated edge
        let (updated_edge_opt, _) = repository.get_by_key("test_edge_1")?;
        let updated_edge = updated_edge_opt.expect("❌ Updated edge should exist");
        
        let mut updated_files: Vec<File> = updated_edge.attachments
            .iter()
            .map(|path| File::open(path))
            .collect::<std::io::Result<_>>()?;
        assert_eq!(
            updated_files.len(),
            1,
            "❌ Failed to open the log file attachment from updated edge"
        );

        // Read as bytes instead of string
        let mut updated_buffer = Vec::new();
        updated_files[0].read_to_end(&mut updated_buffer)?;
        println!("Updated log file size: {} bytes", updated_buffer.len());
        assert!(!updated_buffer.is_empty(), "❌ Updated log file appears to be empty");

        // Verify updated properties
        assert_eq!(
            updated_edge.light_hyper_node.prime_simple_hyper_edge.name,
            "UpdatedConnection",
            "❌ Name property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_node.prime_simple_hyper_edge.directed,
            true,
            "❌ Directed property not updated"
        );
        assert_eq!(
            updated_edge.light_hyper_node.relationship.edge_properties,
            vec!["weight: 10".to_string(), "type: strong".to_string()],
            "❌ Edge properties not updated"
        );

        // Clean up database
        repository.delete("test_edge_1")?;
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "❌ Database should be empty after deletion");

        Ok(())
    }

}