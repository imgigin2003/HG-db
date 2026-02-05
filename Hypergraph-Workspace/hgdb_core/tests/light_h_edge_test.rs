use hgdb_core::hyper_edge::repository::light_h_edge_repository::LightHyperEdgeRepository;
use hgdb_core::hyper_edge::repository::Repository;
use hgdb_core::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use hgdb_core::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property, PropertyType};
use hgdb_core::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use hgdb_core::hyper_edge::entity::relationship::relationship::Relationship;
use hgdb_core::db_config::BASE_DB_PATH;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::remove_dir_all;

    const SUBFOLDER: &str = "light-hyper-edge";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    #[test]
    fn test_light_h_edge_crud_operation() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("Failed to remove DB directory: {:?}", e).into());
            }
            eprintln!("⚠️ DB directory not found, proceeding with test");
        }

        let repository = LightHyperEdgeRepository::new(DB_PATH.as_str())?;

        let nodes = vec!["v1", "v2", "v3", "v4"]
            .into_iter()
            .map(|id| SimpleHyperEdge {
                id: id.to_string(),
                name: id.to_string(),
                main_properties: vec![],
                traversable: false,
                directed: false,
                head_hyper_nodes: None,
                tail_hyper_nodes: None,
                incidence_matrix: vec![]
            })
            .collect::<Vec<_>>();

        let test_key = "e1";
        let test_edge = LightHyperEdge {
            id: test_key.to_string(),
            prime_simple_hyper_edge: SimpleHyperEdge {
                id: test_key.to_string(),
                name: "test_edge_1".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["linked".to_string()],
                    p_type: PropertyType::Simple,
                }],
                traversable: true,
                directed: false,
                head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone(), nodes[3].clone()])),
                incidence_matrix: vec![]
            },
            structural_properties: vec![
                StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
                    layer_index: None,
                },
                StructuralProperty {
                    address: vec!["101 Pine St".to_string()],
                    layer_index: None,
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

        repository.create(test_key, test_edge.clone())?;

        let retrieved_edge = repository.get_by_key(test_key)?.expect("❌ Edge not found after create");
        assert_eq!(retrieved_edge.id, test_key);
        assert_eq!(retrieved_edge.prime_simple_hyper_edge.name, "test_edge_1");

        let updated_edge = LightHyperEdge {
            id: test_key.to_string(),
            prime_simple_hyper_edge: SimpleHyperEdge {
                id: test_key.to_string(),
                name: "UpdatedConnection".to_string(),
                main_properties: vec![Property {
                    key: "type".to_string(),
                    value: vec!["strongly linked".to_string()],
                    p_type: PropertyType::Main,
                }],
                traversable: false,
                directed: true,
                head_hyper_nodes: Some(Box::new(vec![nodes[0].clone(), nodes[1].clone()])),
                tail_hyper_nodes: Some(Box::new(vec![nodes[2].clone(), nodes[3].clone()])),
                incidence_matrix: vec![]
            },
            structural_properties: vec![
                StructuralProperty {
                    address: vec!["789 Oak St".to_string()],
                    layer_index: None,
                },
                StructuralProperty {
                    address: vec!["101 Pine St".to_string()],
                    layer_index: None,
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
        let retrieved_updated_edge = repository.get_by_key(test_key)?.expect("❌ Updated edge not found");

        assert_eq!(
            retrieved_updated_edge.prime_simple_hyper_edge.name,
            "UpdatedConnection",
            "❌ Name property not updated"
        );
        assert_eq!(
            retrieved_updated_edge.prime_simple_hyper_edge.directed,
            true,
            "❌ Directed property not updated"
        );
        assert_eq!(
            retrieved_updated_edge.relationship.edge_properties,
            vec!["weight: 10".to_string(), "type: weak".to_string()],
            "❌ Edge properties not updated"
        );
        assert_eq!(
            retrieved_updated_edge.structural_properties.len(),
            2,
            "❌ Structural properties not updated"
        );

        repository.delete(test_key)?;
        let deleted_edge = repository.get_by_key(test_key)?;
        assert!(deleted_edge.is_none(), "❌ Edge was not deleted");

        Ok(())
    }
}