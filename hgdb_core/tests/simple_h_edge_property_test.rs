use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository; // Import SimpleHyperEdgeRepository
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType}; // Import SimpleHyperEdge, Property, PropertyType

#[cfg(test)]

mod tests{
    use super::*; // Import everything from the parent scope
    use std::error::Error; // Import Error
    use std::fs::remove_dir_all; // Import remove_dir_all
    use std::time::Duration; // Import Duration
    use std::thread::sleep; // Import sleep

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge/simple-h-edge-property"; // RocksDB path

    #[test]
    fn test_property_crud_operation() -> Result<(), Box<dyn Error>> {
        // Give a small delay to ensure RocksDB releases the lock
        sleep(Duration::from_secs(1));

        // Delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }
        
        // Initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;
        
        // create a property
        let property = Property {
            key: "type".to_string(),
            value: vec!["linked".to_string()],
            p_type: PropertyType::Simple
        };

        // define an edge
        let edge = SimpleHyperEdge {
            id: "test_edge_1".to_string(),
            name: "e1".to_string(),
            main_properties: vec![property],
            traversable: true,
            directed: true,
            head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
            tail_hyper_nodes: Some(Box::new(vec!["v3".to_string()]))
        };

        // create the edge
        repository.create("test_edge_1", &edge)?;

        // retrieve the edge
        let retrieved_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(
            retrieved_edge.main_properties.len(),
            1,
            "❌ Property was not added correctly."
        );
        assert_eq!(
            retrieved_edge.main_properties[0].key,
            "type",
            "❌ Property key mismatch."
        );

        // update the property
        let updated_property = Property {
            key: "type".to_string(),
            value: vec!["not-linked".to_string()],
            p_type: PropertyType::Main
        };
        repository.update_property("test_edge_1", "type", updated_property.clone())?;

        // retrieve the updated edge
        let updated_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(
            updated_edge.main_properties[0].value[0],
            "not-linked",
            "❌ Property value was not updated correctly."
        );
        assert_eq!(
            updated_edge.main_properties[0].p_type,
            PropertyType::Main,
            "❌ Property type was not updated correctly."
        );

        // delete the property
        repository.delete_property("test_edge_1", "type")?;

        // retrieve the edge and verify if its deleted
        let final_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(
            final_edge.main_properties.len(),
            0,
            "❌ Property was not deleted correctly."
        );

    Ok(())
    }
}