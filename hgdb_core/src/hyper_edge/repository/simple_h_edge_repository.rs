use rocksdb::{DB, Options};
use serde_json::{self, to_vec};
use crate::hyper_edge::entity::simple_h_edge::SimpleHyperEdge;
use std::error::Error;  // Import general error trait

#[allow(dead_code)]
pub struct HyperEdgeRepository {
    db: DB,
    db_path: String,
}

impl HyperEdgeRepository {
    /// Constructor for creating a new repository
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {  // Return Boxed error type
        let mut opts = Options::default();
        opts.create_if_missing(true);
        
        // Open RocksDB with the provided path
        let db = DB::open(&opts, db_path)?;

        Ok(HyperEdgeRepository {
            db,
            db_path: db_path.to_string(),
        })
    }

    /// Method to create (insert) a SimpleHyperEdge
    pub fn create(&self, key: &str, edge: &SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Serialize the SimpleHyperEdge to Vec<u8>
        let serialized_edge = to_vec(edge).map_err(|e| {
            eprintln!("Serialization error for edge with key '{}': {:?}", key, e);
            Box::new(e) as Box<dyn Error> // Return as a Boxed error
        })?;

        // Insert the serialized edge into the database
        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    /// Method to retrieve a SimpleHyperEdge by key
    pub fn get_by_key(&self, key: &str) -> Result<Option<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                // Deserialize the SimpleHyperEdge
                let edge: SimpleHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    eprintln!("Deserialization error for key '{}': {:?}", key, e);
                    Box::new(e) as Box<dyn Error> // Return as a Boxed error
                })?;
                Ok(Some(edge))
            }
            None => Ok(None), // If the key is not found, return None
        }
    }

    /// Method to update an existing SimpleHyperEdge (simply calls `create`)
    pub fn update(&self, key: &str, edge: &SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        self.create(key, edge) // Reuses the `create` method since it overwrites existing data
    }

    /// Method to delete a SimpleHyperEdge by key
    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }

    /// Method to retrieve all SimpleHyperEdges in the database
    pub fn get_all(&self) -> Result<Vec<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
        let mut edges = Vec::new(); // Vector to hold all edges

        // Iterate over all key-value pairs in the database
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((_key, value)) => {
                    // Deserialize the value into a SimpleHyperEdge
                    match serde_json::from_slice(&value) {
                        Ok(edge) => edges.push(edge), // Add deserialized edge to the vector
                        Err(e) => {
                            eprintln!("Skipping entry due to deserialization error: {:?}", e);
                            continue; // Skip this entry if deserialization fails
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error iterating over database: {:?}", e); // Log iteration error
                    return Err(Box::new(e)); // Return the error as a Boxed error
                }
            }
        }

        Ok(edges) // Return the list of edges
    }
}