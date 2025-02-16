#[cfg(test)]
mod tests {
    use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};
    use hgdb_core::hyper_edge::repository::light_h_edge_repository::LightHyperEdgeRepository;
    use hgdb_core::hyper_edge::entity::light_h_edge::LightHyperEdge;
    use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
    use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;
    use std::error::Error;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/data"; // Your RocksDB path
    
    #[test]
    fn test_light_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Initialize repository
        let repository = LightHyperEdgeRepository::new(DB_PATH)?;

        // Define test data
        let test_key = "test edge";
        let test_edge = LightHyperEdge {
            id: "edge1".to_string(),
            simple_hyper_edge: SimpleHyperEdge {
                id: "edge1".to_string(),
                name: "Friendship".to_string(),
                main_properties: vec![
                    Property {
                        key: "relationship-type".to_string(),
                        value: vec!["friends".to_string()]
                    }
                ],
                traversable: false,
                head_hyper_nodes: Box::new(vec!["alice".to_string(), "bob".to_string()]),
                tail_hyper_nodes: Box::new(vec!["charlie".to_string()]),
            },
            structural_properties: vec![
                StructuralProperty {
                    address: "123 Friendship st".to_string()
                }
            ],
            relationship: Relationship {
                node_1: "alice".to_string(),
                node_2: "bob".to_string(),
                edge_properties: vec!["friends".to_string()]
            },
            traverse: Traverse {
                path: vec!["start".to_string(), "friendship".to_string(), "end".to_string()]
            }
            
        };
        // Create
        repository.create(test_key, &test_edge)?;

        // Retrieve all edges and assert there's at least one
        let all_edges_before = repository.get_all()?;
        assert!(all_edges_before.len() >= 1, "Not all edges were retrieved");

        // Retrieve by key again and verify
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database");
        assert_eq!(retrieved_edge.unwrap().id, "edge1", "Retrieved edge ID mismatch");

        // Update
        let updated_edge = LightHyperEdge {
            id: "edge1".to_string(),  // Keep the same ID for the edge
            // Update the SimpleHyperEdge part of the LightHyperEdge
            simple_hyper_edge: SimpleHyperEdge {
                id: "updated_simple_edge_id".to_string(),  // New SimpleHyperEdge ID
                name: "UpdatedSimpleEdge".to_string(),  // New name for the SimpleHyperEdge
                main_properties: vec![
                    Property {
                        key: "relationship-type".to_string(),  // If you want to update properties
                        value: vec!["close friends".to_string()]
                    }
                ],
                traversable: true,  // Now traversable
                head_hyper_nodes: Box::new(vec!["alice".to_string(), "bob".to_string()]),  // Update head nodes
                tail_hyper_nodes: Box::new(vec!["charlie".to_string()]),  // Update tail nodes
            },
            // Update the structural properties
            structural_properties: vec![
                StructuralProperty {
                    address: "updated_address".to_string(),  // Update address
                }
            ],
            // Update the relationship (nodes and edge properties)
            relationship: Relationship {
                node_1: "alice".to_string(),  // Keep node_1 as Alice
                node_2: "charlie".to_string(),  // Keep node_2 as Charlie
                edge_properties: vec!["updated_properties".to_string()],  // Update relationship properties
            },
            // Update the traverse path
            traverse: Traverse {
                path: vec!["updated_path".to_string(), "friendship_evolution".to_string()],  // Update traverse path
            }
        };
        // Update the repository with the new edge
        repository.update(test_key, &updated_edge)?;        

        // Retrieve and verify the updated edge
        let retrieved_updated_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_updated_edge.is_some(), "Updated edge was not found");
        
        let expected = vec![StructuralProperty { address: "updated_address".to_string() }];
        assert_eq!(retrieved_updated_edge.unwrap().structural_properties, expected, "Update failed");

        // Delete
        repository.delete(test_key)?;
        
        // Debugging line to check the state before and after delete
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "Edge was not deleted");

        let all_edges_after_delete = repository.get_all()?;
        println!("Edges after delete: {:?}", all_edges_after_delete); // Debugging line
        assert_eq!(all_edges_after_delete.len(), 0, "There should be no edges after deletion");

        Ok(())
    }
}