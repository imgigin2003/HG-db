use serde::{Serialize, Deserialize}; // For serialization and deserialization
use std::hash::Hash; // For hashing
use std::borrow::Cow; // import Clone-on-Write
use crate::hyper_edge::entity::h_edge::simple_h_edge::{SimpleHyperEdge, Property}; // import the SimpleHyperEdge and Property structs

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Hash)]
// This struct represents a dual hyper edge in the graph database.
pub struct DualHyperEdge<'a, T, K, V>
where 
    T: Eq + Hash + ToString + std::clone::Clone,
    K: Eq + Hash + std::clone::Clone,
    V: Eq + Hash + std::clone::Clone,
{
    pub id: T, // Unique identifier for the dual hyper edge
    pub name: T, // Name of the dual hyper edge
    pub prime_simple_hyper_edge: Cow<'a, SimpleHyperEdge<T, K, V>>, // Borrowed reference with lifetime 'a
    pub dual_properties: Vec<Property<K, V>>, // Properties associated with the dual hyper edge
    pub traversable: bool, // Indicates if the dual hyper edge is traversable
    pub head_hyper_nodes: Box<Vec<T>>, // List of head hyper nodes associated with this dual hyper edge
    pub tail_hyper_nodes: Option<Box<Vec<T>>>, // Optional list of tail hyper nodes associated with this dual hyper edge
    pub incidence_matrix: Vec<Vec<i8>>, // Stores the incidence matrix
    pub transposed_matrix: Vec<Vec<i8>>, // Stores the transposed incidence matrix
}