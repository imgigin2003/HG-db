use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};
use hgdb_core::hyper_edge::repository::light_h_edge_repository::LightHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::light_h_edge::LightHyperEdge;
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/light-h-edge"; // Your RocksDB path

    #[test]
    fn test_light_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        //delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove DB directory: {:?}", e).into());
            }
            eprintln!("⚠️ DB directory not found, proceeding with test");
        }

        // Initialize repository
        let repository = LightHyperEdgeRepository::new(DB_PATH)?;

        // Define test data
        let test_key = "e1";
        let test_edge = LightHyperEdge {
            id: test_key.to_string(),
            simple_hyper_edge: SimpleHyperEdge {
                id: test_key.to_string(),
                name: "test_edge_1".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()]
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Box::new(vec!["v1".to_string()]),
                tail_hyper_nodes: None,
            },
            structural_properties: vec![
                StructuralProperty {
                    address: vec!["123 Main St".to_string(), "Apt 4B".to_string()]
                }
            ],
            relationship: Relationship {
                node_1: "v1".to_string(),
                node_2: "v2".to_string(),
                directed: true,
                edge_properties: vec!["weight: 5".to_string(), "type: strong".to_string()]
            },
            traverse: Traverse {
                path: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()]
            }
        };

        // Create entry
        repository.create(test_key, &test_edge)?;
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "❌ Edge was not found");

        // Create updated hyperedge
        let updated_edge = LightHyperEdge {
            id: test_key.to_string(),
            simple_hyper_edge: SimpleHyperEdge {
                id: test_key.to_string(),
                name: "UpdatedConnection".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["strongly linked".to_string()],
                }],
                traversable: false,
                directed: true, //set the directed flag to true
                head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
                tail_hyper_nodes: Some(Box::new(vec!["v3".to_string(), "v4".to_string()])),
            },
            structural_properties: vec![
                StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
                },
                StructuralProperty {
                    address: vec!["101 Pine St".to_string()],
                },
            ],
            relationship: Relationship {
                node_1: "v2".to_string(),
                node_2: "v3".to_string(),
                directed: true,
                edge_properties: vec!["weight: 10".to_string(), "type: weak".to_string()],
            },
            traverse: Traverse {
                path: vec!["v2".to_string(), "v3".to_string(), "v4".to_string()],
            },
        };

        repository.update(test_key, &updated_edge)?;
        let retrieved_updated_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_updated_edge.is_some(), "❌ Updated edge not found");

        repository.delete(test_key)?;
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "❌ Edge was not deleted");

        Ok(())
    }
}
