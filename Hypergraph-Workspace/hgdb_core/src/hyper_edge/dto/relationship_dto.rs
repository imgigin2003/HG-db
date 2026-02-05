use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RelationshipDto {
    pub node_1: String,
    pub node_2: String,
    pub directed: bool,
    pub edge_properties: Vec<String>,
}