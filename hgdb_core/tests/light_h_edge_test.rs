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
        // Ensure clean DB state
        if let Err(e) = remove_dir_all(DB_PATH) {
            eprintln!("⚠️ Warning: Could not delete DB directory: {:?}", e);
        }

        // Initialize repository
        let repository = LightHyperEdgeRepository::new(DB_PATH)?;

        // Define test data
        let test_key = "e1";
        let test_edge = LightHyperEdge {
            id: test_key.to_string(),
            simple_hyper_edge: SimpleHyperEdge {
                id: test_key.to_string(), // 🔹 Ensure consistency
                name: "Friendship".to_string(),
                main_properties: vec![
                    Property {
                        key: "relationship-type".to_string(),
                        value: vec!["friends".to_string()]
                    }
                ],
                traversable: false,
                head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string(), "v3".to_string()]),
                tail_hyper_nodes: Box::new(vec!["v4".to_string(), "v5".to_string()]),
            },
            structural_properties: vec![
                StructuralProperty {
                    address: "123 Friendship st".to_string()
                }
            ],
            relationship: Relationship {
                node_1: "v1".to_string(),
                node_2: "v2".to_string(),
                edge_properties: vec!["friends".to_string()]
            },
            traverse: Traverse {
                path: vec!["start".to_string(), "friendship".to_string(), "end".to_string()]
            }
        };

        // Create entry
        repository.create(test_key, &test_edge)?;

        // Retrieve and verify
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "❌ Edge was not found in database");

        // Debugging output
        println!("✅ Retrieved Edge Before Update: {:?}", retrieved_edge);

        assert_eq!(
            retrieved_edge.as_ref().unwrap().id,
            test_key,
            "❌ Retrieved edge ID mismatch"
        );

        // Update
        let updated_edge = LightHyperEdge {
            id: test_key.to_string(),
            simple_hyper_edge: SimpleHyperEdge {
                id: "e2".to_string(),
                name: "UpdatedSimpleEdge".to_string(),
                main_properties: vec![
                    Property {
                        key: "relationship-type".to_string(),
                        value: vec!["best friends".to_string()]
                    }
                ],
                traversable: true,
                head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string(), "v3".to_string()]),
                tail_hyper_nodes: Box::new(vec!["v4".to_string(), "v5".to_string()]),
            },
            structural_properties: vec![
                StructuralProperty {
                    address: "updated_address".to_string()
                }
            ],
            relationship: Relationship {
                node_1: "v1".to_string(),
                node_2: "v3".to_string(),
                edge_properties: vec!["updated_properties".to_string()]
            },
            traverse: Traverse {
                path: vec!["updated_path".to_string(), "friendship_evolution".to_string()]
            }
        };

        repository.update(test_key, &updated_edge)?;

        // Retrieve and verify the updated edge
        let retrieved_updated_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_updated_edge.is_some(), "❌ Updated edge was not found");

        // Debugging output
        println!("✅ Retrieved Edge After Update: {:?}", retrieved_updated_edge);

        let expected = vec![StructuralProperty {
            address: "updated_address".to_string(),
        }];
        assert_eq!(
            retrieved_updated_edge.unwrap().structural_properties,
            expected,
            "❌ Update failed"
        );

        // Delete
        repository.delete(test_key)?;

        // Check deletion
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "❌ Edge was not deleted");

        let all_edges_after_delete = repository.get_all()?;
        println!("✅ Edges after delete: {:?}", all_edges_after_delete);
        assert_eq!(all_edges_after_delete.len(), 0, "❌ There should be no edges after deletion");

        Ok(())
    }
}
