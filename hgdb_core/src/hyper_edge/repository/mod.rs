pub mod simple_h_edge_repository;
pub mod light_h_edge_repository;
pub mod h_edge_repository;

pub trait Repository<T> {
    fn create(&self, entity:&T) -> Result<(), Box<dyn std::error::Error>>;
    fn get_by_key(&self, key:&str) -> Option<T>;
    fn get_all(&self) -> Vec<T>;
    fn update(&self, key:&str, entity:&T) -> Result<(), Box<dyn std::error::Error>>;
    fn update_and_get(&self, key:String, entity:T) -> Result<T, Box<dyn std::error::Error>>;
    fn delete_by_key(&self, key:&str) -> Result<(), Box<dyn std::error::Error>>;
    fn delete_all(&self, keys: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;

}