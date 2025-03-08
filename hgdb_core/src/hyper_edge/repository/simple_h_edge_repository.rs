use rocksdb::{DB, Options}; // Import RocksDB
use serde_json::{self, to_string_pretty}; // Import serde_json
use crate::hyper_edge::entity::h_edge::simple_h_edge::SimpleHyperEdge; // Import SimpleHyperEdge
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge; // Import DualHyperEdge
use std::error::Error; // Import Error trait

#[allow(dead_code)]
// SimpleHyperEdgeRepository struct
pub struct SimpleHyperEdgeRepository {
    pub db: DB,
    db_path: String,
}

// Implementation of SimpleHyperEdgeRepository
impl SimpleHyperEdgeRepository {
    /// Constructor for creating a new repository
    pub fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {  // Return Boxed error type
        let mut opts = Options::default();
        opts.create_if_missing(true);
        
        // Open RocksDB with the provided path
        let db = DB::open(&opts, db_path)?;

        Ok(SimpleHyperEdgeRepository {
            db,
            db_path: db_path.to_string(),
        })
    }

    /// Method to create (insert) a SimpleHyperEdge
    pub fn create(&self, key: &str, edge: &SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Serialize the SimpleHyperEdge to Vec<u8>
        let serialized_edge = to_string_pretty(edge).map_err(|e| {
            eprintln!("❌ Serialization error for edge with key '{}': {:?}", key, e);
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
                    eprintln!("❌ Deserialization error for key '{}': {:?}", key, e);
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
        let mut edges = Vec::new();

        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            match item {
                Ok((_key, value)) => {
                    // Only attempt to deserialize as SimpleHyperEdge
                    match serde_json::from_slice::<SimpleHyperEdge<String, String, String>>(&value) {
                        Ok(edge) => edges.push(edge),
                        Err(e) => {
                            eprintln!("❌ Skipping entry due to deserialization error: {:?}", e);
                            continue; // Skip corrupted or mismatched entries
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
    
    // method to get the dual edge by key
    pub fn get_dual_by_key(&self, key: &str) -> Result<Option<DualHyperEdge<String, String, String>>, Box<dyn Error>> {
        match self.db.get(key)? {
            Some(serialized_edge) => {
                let edge: DualHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge)?;
                println!("🔍 Retrieving Dual Hyperedge"); // Debug log
                Ok(Some(edge))
            }
            None => {
                println!("❌ No Dual Hyperedge found for key: {}", key); // Debug log
                Ok(None)
            }
        }
    }             

    // method to save the dual edge key
    pub fn save_dual(&self, dual_edge: DualHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let key = dual_edge.id.to_string().clone();
        println!("💾 Saving Dual Hyperedge with Key: {}", key); // Debug log
        
        let serialized_dual_edge = to_string_pretty(&dual_edge)?;
        self.db.put(&key, serialized_dual_edge)?;
        println!("✅ Successfully saved Dual Hyperedge with Key: {}", key); // Debug log
    
        Ok(())
    } 
          
}