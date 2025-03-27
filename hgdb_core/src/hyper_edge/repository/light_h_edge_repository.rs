use rocksdb::{DB, Options}; // import the necessary modules from rocksdb
use serde_json::{self, to_string_pretty}; // import the necessary modules from serde_json
use crate::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge; // import the LightHyperEdge struct from the hyper_edge module
use std::error::Error; // import the Error trait from the std::error module

#[allow(dead_code)]
// LightHyperEdgeRepository is a struct that represents a repository for LightHyperEdge entities.
pub struct LightHyperEdgeRepository {
    db: DB,
    db_path: String
} 

// It contains a database instance and a path to the database.
impl LightHyperEdgeRepository {
    // It provides methods for creating, retrieving, updating, and deleting LightHyperEdge entities in the database.
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {
        // Create a new instance of LightHyperEdgeRepository with the given database path.
        let mut opts = Options::default();
        // Set the database path and options for the database.
        opts.create_if_missing(true);

        // Open the database at the given path with the specified options.
        let db = DB::open(&opts, db_path)?;

        // Return a new instance of LightHyperEdgeRepository with the database and path.
        Ok(LightHyperEdgeRepository {
            db, 
            db_path: db_path.to_string()
        })
    }

    // The create method takes a key and a LightHyperEdge entity as parameters and stores the entity in the database with the given key.
    pub fn create(&self, key: &str, edge: &LightHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Serialize the LightHyperEdge entity to a JSON string.
        let serialized_edge = to_string_pretty(edge).map_err(|e| {
            // If serialization fails, print an error message and return a boxed error.
            eprintln!("❌ Serialization error for edge with key '{}': {:?}", key, e);
            // Return a boxed error.
            Box::new(e) as Box<dyn Error>
        })?;

        // Store the serialized LightHyperEdge entity in the database with the given key.
        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    // The get_by_key method takes a key as a parameter and retrieves the LightHyperEdge entity associated with the key from the database.
    pub fn get_by_key(&self, key: &str) -> Result<Option<LightHyperEdge<String, String, String>>, Box<dyn Error>> {
        // Retrieve the serialized LightHyperEdge entity from the database with the given key.
        match self.db.get(key)? {
            // If the serialized entity is found, deserialize it to a LightHyperEdge entity.
            Some(serialized_edge) => {
                // Deserialize the serialized LightHyperEdge entity to a LightHyperEdge entity.
                let edge: LightHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge).map_err(|e| {
                    // If deserialization fails, print an error message and return a boxed error.
                    eprintln!("❌ Deserialization error for key '{}': {:?}", key, e);
                    // Return a boxed error.
                    Box::new(e) as Box<dyn Error>
                })?;
                Ok(Some(edge))
            }
            None => Ok(None)
        }
    }

    // The get_all method retrieves all LightHyperEdge entities from the database and returns them as a vector.
    pub fn get_all(&self) -> Result<Vec<LightHyperEdge<String, String, String>>, Box<dyn Error>> {
        // Retrieve all LightHyperEdge entities from the database and return them as a vector.
        let mut edges = Vec::new();

        // Iterate over all entries in the database.
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((_key, value)) => {
                    // Deserialize the serialized LightHyperEdge entity to a LightHyperEdge entity.
                    match serde_json::from_slice(&value) {
                        Ok(edge) => edges.push(edge),
                        Err(e) => {
                            // If deserialization fails, print an error message and continue to the next entry.
                            eprintln!("❌ Skipping entry due to the deserialization error: {:?}", e);
                            continue;
                        }
                    }
                }

                Err(e) => {
                    // If an error occurs while iterating over the database, print an error message and return a boxed error.
                    eprintln!("❌ Error iterating over database: {:?}", e);
                    // Return a boxed error.
                    return Err(Box::new(e));
                }
            }
        }

        Ok(edges)
    }

    // The update method takes a key and a LightHyperEdge entity as parameters and updates the entity in the database with the given key.
    pub fn update(&self, key: &str, edge: &LightHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        self.create(key, edge)
    }
    // The delete method takes a key as a parameter and deletes the LightHyperEdge entity associated with the key from the database.
    pub fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }
}