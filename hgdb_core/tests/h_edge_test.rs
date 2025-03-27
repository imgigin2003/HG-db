use hgdb_core::hyper_edge::repository::h_edge_repository::HyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::hyper_edge::HyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;
use std::error::Error;
use std::fs::{self, File};

const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/hyper-edge";

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_hyper_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        let repository = HyperEdgeRepository::new(DB_PATH)?;

        let log_file_path = fs::read_dir(DB_PATH)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| path.is_file() && path.extension().map_or(false, |ext| ext == "log"))
            .ok_or_else(|| format!("❌ No .log file found in {}", DB_PATH))?;

        println!("Using log file: {}", log_file_path.display());

        let nodes = vec![
            "v1", "v2", "v3", "v4"
        ].into_iter().map(|id| SimpleHyperEdge {
            id: id.to_string(),
            name: id.to_string(),
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
        }).collect::<Vec<_>>();

        let edges = vec![
            HyperEdge {
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
            },
        ];

        for edge in &edges {
            repository.create(&edge.id, edge)?;
            println!("✅ Successfully created edge: {}", edge.id);
        }

        let (retrieved_edge_opt, _) = repository.get_by_key("test_edge_1")?;
        let retrieved_edge = retrieved_edge_opt.expect("❌ Initial edge should exist");

        let mut files: Vec<File> = retrieved_edge.attachments
            .iter()
            .map(|path| File::open(path))
            .collect::<std::io::Result<_>>()?;
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

        let (updated_edge_opt, _) = repository.get_by_key("test_edge_1")?;
        let updated_edge = updated_edge_opt.expect("❌ Updated edge should exist");

        let mut updated_files: Vec<File> = updated_edge.attachments
            .iter()
            .map(|path| File::open(path))
            .collect::<std::io::Result<_>>()?;
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