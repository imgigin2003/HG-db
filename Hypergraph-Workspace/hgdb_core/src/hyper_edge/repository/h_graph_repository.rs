use crate::hyper_edge::entity::h_graph::layered_hypergraph::LayeredHypergraph;
use crate::hyper_edge::repository::Repository;
use rocksdb::{Options, DB};
use serde_json;
use std::error::Error;
use std::path::Path;

#[allow(unused)]
pub struct LayeredHypergraphRepository {
    pub db: DB,
    db_path: String,
}

impl LayeredHypergraphRepository {
    pub fn open(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        if let Some(parent) = Path::new(db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = DB::open(&opts, db_path)?;
        Ok(Self {
            db,
            db_path: db_path.to_string(),
        })
    }
}

impl Repository<LayeredHypergraph> for LayeredHypergraphRepository {
    fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        Self::open(db_path)
    }

    fn create(&self, key: &str, hypergraph: LayeredHypergraph) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&hypergraph)?;
        self.db.put(key, serialized)?;
        Ok(())
    }

    fn get_by_key(&self, key: &str) -> Result<Option<LayeredHypergraph>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized) => {
                let hypergraph: LayeredHypergraph = serde_json::from_slice(&serialized)?;
                Ok(Some(hypergraph))
            }
            None => Ok(None),
        }
    }

    fn update(&self, key: &str, hypergraph: &LayeredHypergraph) -> Result<(), Box<dyn Error>> {
        if self.db.get(key)?.is_none() {
            return Err(format!("Key {} not found", key).into());
        }
        let serialized = serde_json::to_string(hypergraph)?;
        self.db.put(key, serialized)?;
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<LayeredHypergraph>, Box<dyn Error>> {
        let mut hypergraphs = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (_, value) = item?;
            let hypergraph: LayeredHypergraph = serde_json::from_slice(&value)?;
            hypergraphs.push(hypergraph);
        }
        Ok(hypergraphs)
    }
}
