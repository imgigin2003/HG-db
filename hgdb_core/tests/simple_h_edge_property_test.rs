use hgdb_core::hyper_edge::repository::simple_h_edge_repository::SimpleHyperEdgeRepository;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const DB_PATH: &str = "/users/gigin/documents/mydbs/rocksdb/simple-h-edge-property";

    #[test]
    fn test_simple_h_edge_property_crud_operation() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!("⚠️ Failed to remove DB directory: {:?}", e);
            }
        }

        let repository = SimpleHyperEdgeRepository::new(DB_PATH)?;

        let nodes = vec![
            "v1", "v2", "v3"
        ].into_iter().map(|id| SimpleHyperEdge {
            id: id.to_string(),
            name: id.to_string(),
            main_properties: vec![],
            traversable: false,
            directed: false,
            head_hyper_nodes: None,
            tail_hyper_nodes: None,
        }).collect::<Vec<_>>();

        let property = Property {
            key: "type".to_string(),
            value: vec!["linked".to_string()],
            p_type: PropertyType::Simple,
        };

        let edge = SimpleHyperEdge {
            id: "test_edge_1".to_string(),
            name: "e1".to_string(),
            main_properties: vec![property],
            traversable: true,
            directed: true,
            head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
            tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone()])),
        };

        repository.create("test_edge_1", &edge)?;

        let retrieved_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(retrieved_edge.main_properties.len(), 1, "❌ Property was not added correctly.");
        assert_eq!(retrieved_edge.main_properties[0].key, "type", "❌ Property key mismatch.");

        let updated_property = Property {
            key: "type".to_string(),
            value: vec!["not-linked".to_string()],
            p_type: PropertyType::Main,
        };
        repository.update_property("test_edge_1", "type", updated_property.clone())?;

        let updated_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(updated_edge.main_properties[0].value[0], "not-linked", "❌ Property value was not updated correctly.");
        assert_eq!(updated_edge.main_properties[0].p_type, PropertyType::Main, "❌ Property type was not updated correctly.");

        repository.delete_property("test_edge_1", "type")?;

        let final_edge = repository.get_by_key("test_edge_1")?.unwrap();
        assert_eq!(final_edge.main_properties.len(), 0, "❌ Property was not deleted correctly.");

        Ok(())
    }
}