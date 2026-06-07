use serde::{Deserialize, Serialize};
use std::hash::Hash;
use crate::hyper_edge::entity::h_edge::light_h_edge::LightHyperEdge;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HyperEdge<T: Eq + Hash + Clone, U: Eq + Hash, V: Eq + Hash> {
    pub id: T,
    pub light_hyper_edge: LightHyperEdge<T, U, V>,
    pub attachments: Vec<String>,
}