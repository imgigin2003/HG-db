use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Relationship<T, U, V> {
    pub node_1: T,
    pub node_2: U,
    pub edge_properties: Vec<V>
}