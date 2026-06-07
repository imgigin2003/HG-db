use rocksdb::{DB, Options};
use serde_json::{self, to_string_pretty};
use crate::hyper_edge::entity::h_edge::simple_h_edge::{Property, PropertyType, SimpleHyperEdge};
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge;
use crate::hyper_edge::repository::*;
use std::error::Error;

#[allow(dead_code)]
pub struct SimpleHyperEdgeRepository {
    pub db: DB,
    db_path: String,
}

impl Repository<SimpleHyperEdge<String, String, String>> for SimpleHyperEdgeRepository {
    fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        
        let db = DB::open(&opts, db_path)?;
        Ok(SimpleHyperEdgeRepository {
            db,
            db_path: db_path.to_string(),
        })
    }

    fn create(&self, key: &str, mut edge: SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        ["layer", "level", "aspect"].iter().for_each(|prop_key| {
            if !edge.main_properties.iter().any(|p| p.key == *prop_key) {
                edge.main_properties.push(Property {
                    key: prop_key.to_string(),
                    value: vec!["0".to_string()],
                    p_type: PropertyType::Main
                });
            }
        });

        edge.incidence_matrix = self.create_incidence_matrix(&edge);

        let serialized_edge = to_string_pretty(&edge).map_err(|e| {
            eprintln!("Serialization error for edge {}: {:?}", key, e);
            Box::new(e) as Box<dyn Error>
        })?;

        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    fn get_by_key(&self, key: &str) -> Result<Option<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: SimpleHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    eprintln!("Deserialization error for key '{}': {:?}", key, e);
                    Box::new(e) as Box<dyn Error>
                })?;
                Ok(Some(edge))
            }
            None => Ok(None),
        }
    }

    fn update(&self, key: &str, edge: &SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        self.create(key, edge.clone())
    }

    fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
        let mut edges = Vec::new();

        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((_key, value)) => {
                    match serde_json::from_slice::<SimpleHyperEdge<String, String, String>>(&value) {
                        Ok(edge) => edges.push(edge),
                        Err(e) => {
                            eprintln!("Skipping entry due to deserialization error: {:?}", e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error iterating over database: {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(edges)
    }

    fn save_dual(&self, dual_edge: DualHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let key = dual_edge.id.to_string();
        let serialized_dual_edge = to_string_pretty(&dual_edge)?;
        self.db.put(&key, serialized_dual_edge)?;
        Ok(())
    }

    fn get_dual_by_key(&self, key: &str) -> Result<Option<DualHyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: DualHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge)?;
                Ok(Some(edge))
            }
            None => {
                Ok(None)
            }
        }
    }
}

impl SimpleHyperEdgeRepository {
    pub fn add_property(&self, key: &str, property: Property<String, String>) -> Result<(), Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let mut edge: SimpleHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge)?;
                edge.main_properties.push(property);
                self.create(key, edge)?;
                Ok(())
            }
            None => {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("No Hyperedge found for key: {}", key))))
            }
        }
    }

    pub fn get_property(&self, key: &str) -> Result<Option<Vec<Property<String, String>>>, Box<dyn Error>> {
        if let Some(edge) = self.get_by_key(key)? {
            Ok(Some(edge.main_properties))
        } else {
            Ok(None)
        }
    }

    pub fn update_property(&self, key: &str, property_key: &str, new_property: Property<String, String>) -> Result<(), Box<dyn Error>> {
        let mut edge = self.get_by_key(key)?.ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Edge not found")))?;
        let property = edge.main_properties.iter_mut()
            .find(|p| p.key == property_key)
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Property not found")))?;
        *property = new_property;
        self.update(key, &edge)?;
        Ok(())
    }

    pub fn update_single_layer_property(&self, key: &str, property_key: &str, value: String) -> Result<(), Box<dyn Error>> {
        let mut edge = self.get_by_key(key)?.ok_or_else(|| {
            Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Edge not found"))
        })?;

        if let Some(prop) = edge.main_properties.iter_mut().find(|p| p.key == property_key) {
            prop.value = vec![value];
        } else {
            edge.main_properties.push(Property {
                key: property_key.to_string(),
                value: vec![value],
                p_type: PropertyType::Main
            });
        }

        self.update(key, &edge)?;
        Ok(())
    }

    pub fn get_level(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        if let Some(edge) = self.get_by_key(key)? {
            Ok(edge.main_properties
                .iter()
                .find(|p| p.key == "level")
                .and_then(|p| p.value.first().cloned()))
        } else {
            Ok(None)
        }
    }

    pub fn update_level(&self, key: &str, level: String) -> Result<(), Box<dyn Error>> {
        self.update_single_layer_property(key, "level", level)
    }

    pub fn get_layer(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        if let Some(edge) = self.get_by_key(key)? {
            Ok(edge.main_properties
            .iter()
            .find(|p| p.key == "layer")
            .and_then(|p| p.value.first().cloned()))
        } else {
            Ok(None)
        }
    }

    pub fn update_layer(&self, key: &str, layer: String) -> Result<(), Box<dyn Error>> {
        self.update_single_layer_property(key, "layer", layer)
    }

    pub fn delete_property(&self, key: &str, property_key: &str) -> Result<(), Box<dyn Error>> {
        if let Some(mut edge) = self.get_by_key(key)? {
            if let Some(pos) = edge.main_properties.iter().position(|p| p.key == property_key) {
                edge.main_properties.remove(pos);
                self.update(key, &edge)?;
                Ok(())
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Property not found")))
            }
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Edge not found")))
        }
    }

    pub fn save(&self, edge: SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let key = edge.id.clone();
        let mut edge_with_matrix = edge;
        edge_with_matrix.incidence_matrix = self.create_incidence_matrix(&edge_with_matrix);
        self.print_matrix(&edge_with_matrix.incidence_matrix, &format!("Incidence Matrix for {}", key));
        
        let serialized_edge = to_string_pretty(&edge_with_matrix).map_err(|e| {
            eprintln!("Serialization error for edge with key '{}': {:?}", key, e);
            Box::new(e) as Box<dyn Error>
        })?;
        self.db.put(&key, serialized_edge)?;
        Ok(())
    }

    pub fn create_incidence_matrix(&self, edge: &SimpleHyperEdge<String, String, String>) -> Vec<Vec<i8>> {
        let mut nodes_set: Vec<String> = Vec::new();
        
        if let Some(head_node_ids) = &edge.head_hyper_nodes {
            nodes_set.extend(head_node_ids.clone());
        }
        if let Some(tail_node_ids) = &edge.tail_hyper_nodes {
            nodes_set.extend(tail_node_ids.clone());
        }
        
        nodes_set.sort();
        nodes_set.dedup();

        let mut matrix = vec![vec![0i8; 1]; nodes_set.len()];
        
        let head_ids = edge.head_hyper_nodes.as_ref().map_or(vec![], |ids| ids.clone());
        let tail_ids = edge.tail_hyper_nodes.as_ref().map_or(vec![], |ids| ids.clone());

        for (i, node) in nodes_set.iter().enumerate() {
            let is_in_head = head_ids.contains(node);
            let is_in_tail = tail_ids.contains(node);
            matrix[i][0] = match (is_in_head, is_in_tail) {
                (true, false) => 1,
                (false, true) => 2,
                (true, true) => 3,
                (false, false) => 0,
            };
        }
        matrix
    }

    pub fn print_matrix(&self, matrix: &Vec<Vec<i8>>, label: &str) {
        println!(
            "Matrix {} [{}x{}]:",
            label,
            matrix.len(),
            if matrix.is_empty() { 0 } else { matrix[0].len() }
        );
        for row in matrix {
            let row_str: String = row
                .iter()
                .map(|&val| val.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            println!("[ {} ]", row_str);
        }
    }
}