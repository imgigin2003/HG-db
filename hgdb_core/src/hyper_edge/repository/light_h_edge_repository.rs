use rocksdb::{DB, Options};
use serde_json::{self, to_string_pretty};
use crate::hyper_edge::entity::light_h_edge::LightHyperEdge;
use std::error::Error;

#[allow(dead_code)]
pub struct LightHyperEdgeRepository {
    db: DB,
    db_path: String
} 

impl LightHyperEdgeRepository {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, db_path)?;

        Ok(LightHyperEdgeRepository {
            db, 
            db_path: db_path.to_string()
        })
    }

    pub fn create(&self, key: &str, edge: &LightHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let serialized_edge = to_string_pretty(edge).map_err(|e| {
            eprintln!("❌ Serialization error for edge with key '{}': {:?}", key, e);
            Box::new(e) as Box<dyn Error>
        })?;

        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    pub fn get_by_key(&self, key: &str) -> Result<Option<LightHyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: LightHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    eprintln!("❌ Deserialization error for key '{}': {:?}", key, e);
                    Box::new(e) as Box<dyn Error>
                })?;
                Ok(Some(edge))
            }
            None => Ok(None)
        }
    }

    pub fn get_all(&self) -> Result<Vec<LightHyperEdge<String, String, String>>, Box<dyn Error>> {
        let mut edges = Vec::new();

        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((_key, value)) => {
                    match serde_json::from_slice(&value) {
                        Ok(edge) => edges.push(edge),
                        Err(e) => {
                            eprintln!("❌ Skipping entry due to the deserialization error: {:?}", e);
                            continue;
                        }
                    }
                }

                Err(e) => {
                    eprintln!("❌ Error iterating over database: {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }

        Ok(edges)
    }

    pub fn update(&self, key: &str, edge: &LightHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        self.create(key, edge)
    }

    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }
}