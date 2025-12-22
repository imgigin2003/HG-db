use rocksdb::{DB, Options}; // Import RocksDB
use serde_json::{self, to_string_pretty}; // Import serde_json
use crate::hyper_edge::entity::h_edge::hyper_edge::HyperEdge; // Import HyperEdge
use crate::hyper_edge::repository::*; // Import Repository trait
use std::error::Error; // Import Error trait
use std::fs::File; // Import File

/****************************************************** CRUD Operation for Hyper Edge ************************************************+/
                        Here we are going to list all methods regarding creating a new Hyper Edge in the Database
****************************************************************************************************************************/

#[allow(dead_code)]
// HyperEdgeRepository struct
pub struct HyperEdgeRepository {
    pub db: DB,
    db_path: String,
}

// Implementation of HyperEdgeRepository
impl Repository<HyperEdge<String, String, String>> for HyperEdgeRepository {
    /// Constructore for creating a new repository
    fn new(db_path: &str) -> Result<Self, Box<dyn Error>> { 
        // Create a new Options instance
        let mut opts = Options::default();
        // Set the option to create the database if it does not exist
        opts.create_if_missing(true);

        // Open ROCKSDB with the provided path
        let db = DB::open(&opts, db_path)?;
        // Return the HyperEdgeRepository instance
        Ok(HyperEdgeRepository {
            db,
            db_path: db_path.to_string()
        })
    }

    /// Method to create a HyperEdge
    fn create(&self, key: &str, edge: HyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Serialize the HyperEdge
        let serialized_edge = to_string_pretty(&edge).map_err(|e| {
            // Print the error if serialization fails
            eprintln!("❌ Serialization error for edge with key '{}': {:?}", key, e);
            // Return the error as a Box<dyn Error>
            Box::new(e) as Box<dyn Error>
        })?;

        // Insert the serialized edge into the database
        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    /// Method to retrieve a HyperEdge by key
    fn get_by_key(&self, key: &str) -> Result<Option<HyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: HyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    eprintln!("❌ Deserialization error for key '{}': {:?}", key, e);
                    Box::new(e) as Box<dyn Error>
                })?;
                Ok(Some(edge))
            }
            None => Ok(None), // No edge found
        }
    }

    /// Method to update an existing HyperEdge (simply calls `create`)
    fn update(&self, key: &str, edge: &HyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Check if the key exists
        if self.db.get(key)?.is_none() {
            // If the key does not exist, return an error
            return Err(format!("❌ Cannot update: Key '{}' not found", key).into());
        }
        // Call the `create` method to update the HyperEdge
        self.create(key, edge.clone()) 
    }    

    /// Method to delete a HyperEdge by key
    fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        // Match the result of deleting the key from the database
        match self.db.delete(key) {
            Ok(_) => Ok(()),
            // If the key is not found, return an error
            Err(_) => Err(format!("❌ Failed to delete key '{}', possibly not found", key).into()),
        }
    }    

    /// Method to retrieve all HyperEdges
    fn get_all(&self) -> Result<Vec<HyperEdge<String, String, String>>, Box<dyn Error>> {
        // Create a new vector to store the HyperEdges
        let mut edges = Vec::new();
        // Create a new iterator for the database
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        // Iterate over the iterator and deserialize each HyperEdge
        for item in iter {
            // Unwrap the item into a key and value
            let (key, value) = item?;
            // Deserialize the value into a HyperEdge
            let edge: HyperEdge<String, String, String> = serde_json::from_slice(&value).map_err(|e| {
                // Print the error if deserialization fails
                eprintln!("❌ Deserialization error for key '{:?}': {:?}", key, e);
                // Return the error as a Box<dyn Error>
                Box::new(e) as Box<dyn Error>
            })?;
            // Push the HyperEdge into the vector
            edges.push(edge);
        }
        Ok(edges)
    }
}

impl HyperEdgeRepository {
    /// Method to retrieve all HyperEdges by a given source
    pub fn get_by_source(&self, source: &str) -> Result<Vec<HyperEdge<String, String, String>>, Box<dyn Error>> {
        // Create a new vector to store the HyperEdges
        let mut edges = Vec::new();
        // Create a new iterator for the database starting from the beginning
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        // Iterate over the iterator and deserialize each HyperEdge
        for item in iter {
            // Unwrap the item into a key and value
            let (key, value) = item?;
            // Deserialize the value into a HyperEdge
            let edge: HyperEdge<String, String, String> = serde_json::from_slice(&value).map_err(|e| {
                // Print the error if deserialization fails
                eprintln!("❌ Deserialization error for key '{:?}': {:?}", key, e);
                // Return the error as a Box<dyn Error>
                Box::new(e) as Box<dyn Error>
            })?;
            // Check if the source is in the attachments
            if edge.attachments.iter().any(|attachment| attachment.to_string_lossy() == source) {
                // Push the HyperEdge into the vector
                edges.push(edge);
            }
        }
        Ok(edges)
    }

    /// Method to open all attachments as Files for a given HyperEdge
    pub fn open_attachments(&self, edge: &HyperEdge<String, String, String>) -> Result<Vec<Box<File>>, Box<dyn Error>> {
        let files: Vec<Box<File>> = edge.attachments
            .iter()
            .map(|path| File::open(path).map(Box::new)) // Open each path and box it
            .collect::<Result<Vec<_>, _>>()?; // Collect into Result<Vec<Box<File>>, io::Error>
        Ok(files)
    }
}