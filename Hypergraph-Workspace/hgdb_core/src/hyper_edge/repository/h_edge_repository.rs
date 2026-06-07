use rocksdb::{DB, Options};
use serde_json::{self, to_string_pretty};
use crate::hyper_edge::entity::h_edge::hyper_edge::HyperEdge;
use crate::hyper_edge::repository::*;
use std::error::Error;
use std::fs::File;

#[allow(dead_code)]
pub struct HyperEdgeRepository {
    pub db: DB,
    db_path: String,
}

impl Repository<HyperEdge<String, String, String>> for HyperEdgeRepository {
    fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, db_path)?;
        Ok(HyperEdgeRepository {
            db,
            db_path: db_path.to_string()
        })
    }

    fn create(&self, key: &str, edge: HyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let serialized_edge = to_string_pretty(&edge).map_err(|e| {
            eprintln!("Serialization error for edge with key '{}': {:?}", key, e);
            Box::new(e) as Box<dyn Error>
        })?;

        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    fn get_by_key(&self, key: &str) -> Result<Option<HyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: HyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    eprintln!("Deserialization error for key '{}': {:?}", key, e);
                    Box::new(e) as Box<dyn Error>
                })?;
                Ok(Some(edge))
            }
            None => Ok(None),
        }
    }

    fn update(&self, key: &str, edge: &HyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        if self.db.get(key)?.is_none() {
            return Err(format!("Cannot update: Key '{}' not found", key).into());
        }
        self.create(key, edge.clone())
    }

    fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        match self.db.delete(key) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to delete key '{}', possibly not found", key).into()),
        }
    }

    fn get_all(&self) -> Result<Vec<HyperEdge<String, String, String>>, Box<dyn Error>> {
        let mut edges = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        
        for item in iter {
            let (key, value) = item?;
            let edge: HyperEdge<String, String, String> = serde_json::from_slice(&value).map_err(|e| {
                eprintln!("Deserialization error for key '{:?}': {:?}", key, e);
                Box::new(e) as Box<dyn Error>
            })?;
            edges.push(edge);
        }
        Ok(edges)
    }
}

impl HyperEdgeRepository {
    pub fn get_by_source(&self, source: &str) -> Result<Vec<HyperEdge<String, String, String>>, Box<dyn Error>> {
        let mut edges = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        
        for item in iter {
            let (key, value) = item?;
            let edge: HyperEdge<String, String, String> = serde_json::from_slice(&value).map_err(|e| {
                eprintln!("Deserialization error for key '{:?}': {:?}", key, e);
                Box::new(e) as Box<dyn Error>
            })?;
            
            if edge.attachments.iter().any(|attachment| attachment == source) {
                edges.push(edge);
            }
        }
        Ok(edges)
    }

    pub fn open_attachments(&self, edge: &HyperEdge<String, String, String>) -> Result<Vec<Box<File>>, Box<dyn Error>> {
        let files: Vec<Box<File>> = edge.attachments
            .iter()
            .map(|path| File::open(path).map(Box::new))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(files)
    }
}