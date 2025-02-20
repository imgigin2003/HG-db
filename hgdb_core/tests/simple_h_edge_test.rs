use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge"; // RocksDB path

    #[test]
    fn test_simple_h_edge_crud_operations() -> Result<(), Box<dyn Error>> {
        // Delete the database folder before running the test to ensure clean slate
        let _ = remove_dir_all(DB_PATH);

        // Initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;

        // Define test data
        let test_key = "test_edge";
        let test_edge = SimpleHyperEdge {
            id: test_key.to_string(),
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
        };

        // Create
        repository.create(test_key, &test_edge)?;

        // Retrieve all edges and assert there is at least one
        let all_edges = repository.get_all()?;
        assert!(all_edges.len() >= 1, "Not all edges were retrieved");

        // Retrieve by key again and verify
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "Friendship", "Retrieved edge name mismatch");

        // Log retrieved edge for debugging
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database after create");
        println!("Retrieved edge: {:?}", retrieved_edge.unwrap());  // Log the retrieved edge

        // Update
        let updated_edge = SimpleHyperEdge {
            name: "Updated Edge".to_string(),
            ..test_edge.clone() // Clone and update the name
        };
        repository.update(test_key, &updated_edge)?;

        // Retrieve and verify the updated edge
        let retrieved_updated_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_updated_edge.is_some(), "Updated edge was not found");
        assert_eq!(retrieved_updated_edge.unwrap().name, "Updated Edge", "Update failed");

        // Delete
        repository.delete(test_key)?;
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "Edge was not deleted");

        // Ensure the database is empty after deletion
        let all_edges_after_delete = repository.get_all()?;
        assert_eq!(all_edges_after_delete.len(), 0, "There should be no edges after deletion");

        Ok(())
    }
}
