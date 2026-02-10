use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PropertyTypeDto {
    Simple,
    Main,
    Structure,
    ExtraInfo,
}

impl FromStr for PropertyTypeDto {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simple" => Ok(PropertyTypeDto::Simple),
            "main" => Ok(PropertyTypeDto::Main),
            "structure" => Ok(PropertyTypeDto::Structure),
            "extrainfo" => Ok(PropertyTypeDto::ExtraInfo),
            _ => Err(format!("Unknown property type: {}", s)),
        }
    }
}

impl From<String> for PropertyTypeDto {
    fn from(s: String) -> Self {
        s.as_str().parse().unwrap_or(PropertyTypeDto::Simple)
    }
}

impl From<&str> for PropertyTypeDto {
    fn from(s: &str) -> Self {
        s.parse().unwrap_or(PropertyTypeDto::Simple)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PropertyDto {
    pub key: String,
    pub value: Vec<String>,
    pub p_type: PropertyTypeDto,
}
