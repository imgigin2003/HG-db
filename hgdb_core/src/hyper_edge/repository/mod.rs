pub mod simple_h_edge_repository;
pub mod light_h_edge_repository;
pub mod h_edge_repository;

use std::error::Error; // for error handling
use crate::hyper_edge::entity::h_edge::dual_h_edge::DualHyperEdge; // import DualHyperEdge

pub trait Repository<T> 
where 
    T: serde::Serialize + serde::de::DeserializeOwned + 'static,
    {
        /// Create a new repository instance with the given database path
        fn new(db_path: &str) -> Result<Self, Box<dyn Error>> where Self: Sized;

        /// Insert a new entity into the repository under the given key
        fn create(&self, key: &str, entity: T) -> Result<(), Box<dyn Error>>;

        /// Retrieves an entity by its key
        fn get_by_key(&self, key: &str) -> Result<Option<T>, Box<dyn Error>>;

        /// Update an exisiting entity in the repository
        fn update(&self, key: &str, entity: &T) -> Result<(), Box<dyn Error>>;

        /// Delete an entity from the repository
        fn delete(&self, key: &str) -> Result<(), Box<dyn Error>>;

        /// Get all entities in the repository
        fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

        /// Save a dual edge in the repository
        fn save_dual(&self, _dual_edge: DualHyperEdge<String, String, String>) -> Result<(), Box<dyn Error>> {
            Err("Saving dual edges is not supported by this repository".into())
        }

    }