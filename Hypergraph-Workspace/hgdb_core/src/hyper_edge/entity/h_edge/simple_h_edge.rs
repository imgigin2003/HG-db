use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyType {
    Simple,
    Main,
    Structure,
    ExtraInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property<K: Eq + Hash, V: Eq + Hash> {
    pub key: K,
    pub value: Vec<V>,
    pub p_type: PropertyType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimpleHyperEdge<T: Eq + Hash + Clone, K: Eq + Hash, V: Eq + Hash> {
    pub id: T,
    pub name: T,
    pub main_properties: Vec<Property<K, V>>,
    pub traversable: bool,
    pub directed: bool,
    pub head_hyper_nodes: Option<Vec<T>>,
    pub tail_hyper_nodes: Option<Vec<T>>,
    pub incidence_matrix: Vec<Vec<i8>>,
}