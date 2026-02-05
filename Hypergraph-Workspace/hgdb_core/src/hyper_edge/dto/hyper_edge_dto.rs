use serde::{Serialize, Deserialize};
use crate::hyper_edge::dto::light_hyper_edge_dto::LightHyperEdgeResponseDto;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]   
pub struct HyperEdgeResponseDto {
    pub id: String,
    pub light_hyper_edge: LightHyperEdgeResponseDto,
    pub attachments: Vec<String>,
}