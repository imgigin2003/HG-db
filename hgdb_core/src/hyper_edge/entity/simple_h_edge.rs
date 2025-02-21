// Importing necessary traits from serde and std::hash
use serde::{Serialize, Deserialize}; // For serializing and deserializing data
use std::hash::Hash; // For implementing hash-based collections

// Deriving Serialize, Deserialize, and Debug traits for the Property struct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Property<K: Eq + Hash, V: Eq + Hash> {
    pub key: K, // The key for the property
    pub value: Vec<V>, // The value associated with the key
}

// Deriving Serialize, Deserialize, and Debug traits for the SimpleHyperEdge struct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimpleHyperEdge<T: Eq + Hash + ToString, K: Eq + Hash, V: Eq + Hash> {
    pub id: T, // The unique ID for the hyperedge
    pub name: T, // The name of the hyperedge
    pub main_properties: Vec<Property<K,V>>, // A list of properties associated with the hyperedge
    pub traversable: bool, // A flag to mark whether the hyperedge is traversable
    pub directed: bool, // a flag to set wheter a graph is directed or undirected
    pub head_hyper_nodes: Box<Vec<T>>, // A vector of head hypernodes (recursive structure)
    pub tail_hyper_nodes: Option<Box<Vec<T>>>, // A vector of tail hypernodes (recursive structure)
}
