use serde::{Serialize, Deserialize}; // import serialize and deserialize from serde
use std::{path::PathBuf, hash::Hash}; // import Hash from std::hash
use crate::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;
use std::fmt::Display; // import Display from std::fmt

// define a struct HyperEdge with generic types T, U, V
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)] 
pub struct HyperEdge<T: Eq + Hash + Display, U: Eq + Hash, V: Eq + Hash> {
    pub id: T, // must be a combination of important info about hn, for example where it is stored
    pub light_hyper_edge: LightHyperEdge<T, U, V>, // a LightHyperEdge with generic types T, U, V
    pub attachments: Vec<PathBuf> // a vector of PathBuf which is a struct that represents a path in the filesystem
}