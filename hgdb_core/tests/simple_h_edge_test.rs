#[cfg(test)]
mod tests {
    use hgdb_core::hyper_edge::repository::h_edge_repository::HyperEdgeRepository;
    use hgdb_core::hyper_edge::entity::simple_h_edge::{SimpleHyperEdge, Property};
    use std::error::Error;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/data"; // Use your existing RocksDB path

    #[test]
    fn test_crud_operations() -> Result<(), Box<dyn Error>> {
        let repository = HyperEdgeRepository::new(DB_PATH)?;

        let test_key = "test_edge";
        let test_edge = SimpleHyperEdge {
            id: "Keys".to_string(),
            name: "Edges".to_string(),
            main_properties: vec![
                Property { key: "e1".to_string(), value: vec!["v1".to_string(), "v2".to_string(), "v3".to_string()] },
                Property { key: "e2".to_string(), value: vec!["v4".to_string(), "v5".to_string()] },
                Property { key: "e3".to_string(), value: vec!["v6".to_string()]}
            ],
            traversable: true,
            head_hyper_nodes: Box::new(vec![]),
            tail_hyper_nodes: Box::new(vec![]),
        };

        // Create
        repository.create(test_key, &test_edge)?;

        //get all
        let all_edges = repository.get_all()?;
        assert!(all_edges.len() >= 1, "Not all edges were retrieved");

        //get by key again
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database");
        assert_eq!(retrieved_edge.unwrap().name, "Edges", "Retrieved edge name mismatch");

        // Verify creation by calling get_by_key immediately
        let retrieved_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_edge.is_some(), "Edge was not found in database after create");
        println!("Retrieved edge: {:?}", retrieved_edge.unwrap());  // Log the retrieved edge

        // Update
        let updated_edge = SimpleHyperEdge {
            name: "Updated Edge".to_string(),
            ..test_edge.clone()
        };
        repository.update(test_key, &updated_edge)?;

        let retrieved_updated_edge = repository.get_by_key(test_key)?;
        assert!(retrieved_updated_edge.is_some(), "Updated edge was not found");
        assert_eq!(retrieved_updated_edge.unwrap().name, "Updated Edge", "Update failed");

        // Delete
        repository.delete(test_key)?;
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "Edge was not deleted");

        Ok(())
    }
}
