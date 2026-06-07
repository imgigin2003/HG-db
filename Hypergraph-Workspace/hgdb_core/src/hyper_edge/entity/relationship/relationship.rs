use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Relationship<T, U, V> {
    pub node_1: T,
    pub node_2: U,
    pub directed: bool,
    pub edge_properties: Vec<V>,
}