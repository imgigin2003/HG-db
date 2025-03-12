use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::{remove_dir_all, File, metadata};
    use std::io::Write;
    use serde_json::to_string_pretty;
    use std::time::Duration;
    use std::thread::sleep;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge"; // RocksDB path
    const JSON_PATH: &str = "/users/gigin/documents/github/HG-DB/hgdb_core/hg_app/py_scripts/json-data"; // JSON data path

    #[test]
    fn test_simple_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        // Delete the database folder before running the test
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        // Initialize repository
        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;

        // Define test edges with unique keys
        let edges = vec![
            ("test_edge_1", SimpleHyperEdge {
                id: "test_edge_1".to_string(),
                name: "e1".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["linked".to_string()],
                        p_type: PropertyType::Simple
                    }
                ],
                traversable: true,
                directed: true,
                head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
                tail_hyper_nodes: Some(Box::new(vec!["v3".to_string()]))
            }),
            ("test_edge_2", SimpleHyperEdge {
                id: "test_edge_2".to_string(),
                name: "e2".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["not-linked".to_string()],
                        p_type: PropertyType::Simple
                    }
                ],
                traversable: false,
                directed: false,
                head_hyper_nodes: Box::new(vec!["v4".to_string(), "v5".to_string()]),
                tail_hyper_nodes: None
            }),
            ("test_edge_3", SimpleHyperEdge {
                id: "test_edge_3".to_string(),
                name: "e3".to_string(),
                main_properties: vec![
                    Property {
                        key: "type".to_string(),
                        value: vec!["not-linked".to_string()],
                        p_type: PropertyType::ExtraInfo
                    }
                ],
                traversable: true,
                directed: false,
                head_hyper_nodes: Box::new(vec!["v6".to_string(), "v7".to_string(), "v8".to_string()]),
                tail_hyper_nodes: None
            })
        ];

        // Create edges
        for (key, edge) in &edges {
            repository.create(key, edge)?;
        }

        // Retrieve all edges and verify count
        let all_edges = repository.get_all()?;
        assert_eq!(all_edges.len(), edges.len(), "❌ Not all edges were stored correctly");

        // Generate JSON file after operations
        let all_edges_for_json = repository.get_all()?;

        // Serialize the edges to a JSON string
        let json_data = to_string_pretty(&all_edges_for_json)?;

        // Specify the file path for the JSON file
        let json_path = format!("{}/test_simple.json", JSON_PATH);

        // Write the JSON data to a file
        let mut file = File::create(json_path.clone())?;
        file.write_all(json_data.as_bytes())?;

        // Assert the JSON file was created
        let data = metadata(&json_path)?;
        assert!(data.is_file(), "❌ JSON file was not created at expected path");

        // Update an existing edge (test update)
        let updated_edge = SimpleHyperEdge {
            id: "test_edge_1".to_string(),
            name: "e1_updated".to_string(),
            main_properties: vec![
                Property {
                    key: "type".to_string(),
                    value: vec!["updated-linked".to_string()],
                    p_type: PropertyType::Main
                },
            ],
            traversable: false,
            directed: false,
            head_hyper_nodes: Box::new(vec!["v1_updated".to_string()]),
            tail_hyper_nodes: Some(Box::new(vec!["v3_updated".to_string()])),
        };

        // Update the edge in the repository
        repository.update("test_edge_1", &updated_edge)?;

        // Verify update
        let updated_retrieved_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(updated_retrieved_edge.name, "e1_updated", "❌ Updated edge name mismatch");

        // Delete all edges
        for (key, _) in &edges {
            repository.delete(key)?;
        }

        // Verify database is empty
        let all_edges_after_delete = repository.get_all()?;
        assert!(all_edges_after_delete.is_empty(), "❌ Database should be empty after deleting all edges");

        Ok(())
    }

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

        // Give it a moment before reopening the database
        sleep(Duration::from_secs(1));

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
            id: "test_edge".to_string(),
            name: "e1".to_string(),
            main_properties: vec![property],
            traversable: true,
            directed: true,
            head_hyper_nodes: Box::new(vec!["v1".to_string(), "v2".to_string()]),
            tail_hyper_nodes: Some(Box::new(vec!["v3".to_string()]))
        };

        // create the edge
        repository.create("test_edge", &edge)?;

        // retrieve the edge
        let retrieved_edge = repository.get_by_key("test_edge")?.unwrap();
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
        repository.update_property("test_edge", "type", updated_property.clone())?;

        // retrieve the updated edge
        let updated_edge = repository.get_by_key("test_edge")?.unwrap();
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
        repository.delete_property("test_edge", "type")?;

        // retrieve the edge and verify if its deleted
        let final_edge = repository.get_by_key("test_edge")?.unwrap();
        assert_eq!(
            final_edge.main_properties.len(),
            0,
            "❌ Property was not deleted correctly."
        );

    Ok(())
    }
}
