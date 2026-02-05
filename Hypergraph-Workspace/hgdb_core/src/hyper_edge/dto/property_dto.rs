use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PropertyTypeDto {
    Simple,
    Main,
    Structure,
    ExtraInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PropertyDto {
    pub key: String,
    pub value: Vec<String>,
    pub p_type: PropertyTypeDto,
}