use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::hyper_edge::entity::h_graph::layered_hypergraph::LayeredHypergraph;

#[derive(Clone)]
pub struct InMemoryLayeredRepository {
    store: Arc<Mutex<HashMap<String, LayeredHypergraph>>>,
}

impl InMemoryLayeredRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create(&self, id: String, layered: LayeredHypergraph) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();
        store.insert(id, layered);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<LayeredHypergraph>, String> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id).cloned())
    }

    pub fn update(&self, id: String, layered: LayeredHypergraph) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();
        if store.contains_key(&id) {
            store.insert(id, layered);
            Ok(())
        } else {
            Err("Not found".to_string())
        }
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        let mut store = self.store.lock().unwrap();
        store.remove(id);
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<LayeredHypergraph>, String> {
        let store = self.store.lock().unwrap();
        Ok(store.values().cloned().collect())
    }
}