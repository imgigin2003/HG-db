use rocksdb::{DB, Options}; // Import RocksDB
use serde_json::{self, to_string_pretty}; // Import serde_json
use crate::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property}; // Import SimpleHyperEdge
use crate::hyper_edge::repository::*; // Import Repository trait
use std::error::Error; // Import Error trait

/****************************************************** CRUD Operation for Hyper Edge ************************************************+/
                        Here we are going to list all methods regarding creating a new Hyper Edge in the Database
****************************************************************************************************************************/

#[allow(dead_code)]
// SimpleHyperEdgeRepository struct
pub struct SimpleHyperEdgeRepository {
    pub db: DB,
    db_path: String,
}

// Implementation of SimpleHyperEdgeRepository
impl Repository<SimpleHyperEdge<String, String, String>> for SimpleHyperEdgeRepository {
    /// Constructor for creating a new repository
    fn new(db_path: &str) -> Result<Self, Box<dyn Error>> {  // Return Boxed error type
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
    fn create(&self, key: &str, edge: SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        // Serialize the SimpleHyperEdge to Vec<u8>
        let serialized_edge = to_string_pretty(&edge).map_err(|e| {
            eprintln!("❌ Serialization error for edge with key '{}': {:?}", key, e);
            Box::new(e) as Box<dyn Error> // Return as a Boxed error
        })?;

        // Insert the serialized edge into the database
        self.db.put(key, serialized_edge)?;
        Ok(())
    }

    /// Method to retrieve a SimpleHyperEdge by key
    fn get_by_key(&self, key: &str) -> Result<Option<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
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
    fn update(&self, key: &str, edge: &SimpleHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        self.create(key, edge.clone()) // Reuses the `create` method since it overwrites existing data
    }

    /// Method to delete a SimpleHyperEdge by key
    fn delete(&self, key: &str) -> Result<(), Box<dyn Error>> {
        self.db.delete(key)?;
        Ok(())
    }

    /// Method to retrieve all SimpleHyperEdges in the database
    fn get_all(&self) -> Result<Vec<SimpleHyperEdge<String, String, String>>, Box<dyn Error>> {
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

    fn save_dual(&self, dual_edge: DualHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
        let key = dual_edge.id.to_string();
        let serialized_dual_edge = to_string_pretty(&dual_edge)?;
        self.db.put(&key, serialized_dual_edge)?;
        Ok(())
    }
}

impl SimpleHyperEdgeRepository {
    /*******************************************CRUD Operation for Property Field in Hyper Edge ************************************************+/
                        Here we are going to list all methods to Create, Update, Delete and Retrieve a Property Field
****************************************************************************************************************************/
    // Method to add a property to a HyperEdge
    pub fn add_property(&self, key: &str, property: Property<String, String>) -> Result<(), Box<dyn Error>> {
        match self.db.get(key)? { // Retrieve the HyperEdge by key
            Some(serialized_edge) => { // If the HyperEdge is found
                // Deserialize the HyperEdge 
                let mut edge: SimpleHyperEdge<String, String, String> = serde_json::from_slice(&serialized_edge)?;
                edge.main_properties.push(property); // Add the new property
                self.create(key, edge)?; // Reuse create to update the edge in the DB
                Ok(())
            }
            None => {
                // Return an error if no hyperedge was found for the provided key
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("No Hyperedge found for key: {}", key))))
            }
        }
    }

    // Method to Get a property in a HyperEdge
    pub fn get_property(&self, key: &str) -> Result<Option<Vec<Property<String, String>>>, Box<dyn Error>> {
        // Retrieve the HyperEdge by key
        if let Some(edge) = self.get_by_key(key)? {
            // Return the main properties if the HyperEdge is found
            Ok(Some(edge.main_properties))
        } else {
            Ok(None)
        }
    }

    // Method to Update a property
    pub fn update_property(&self, key: &str, property_key: &str, new_property: Property<String, String>) -> Result<(), Box<dyn Error>> {
        let mut edge = self.get_by_key(key)?.ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Edge not found")))?;
        let property = edge.main_properties.iter_mut()
            .find(|p| p.key == property_key)
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Property not found")))?;
        *property = new_property;
        self.update(key, &edge)?;
        Ok(())
    }

    // Method to Delete a property
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
}