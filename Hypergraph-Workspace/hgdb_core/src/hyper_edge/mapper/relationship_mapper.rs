use crate::hyper_edge::entity::relationship::relationship::Relationship;
use crate::hyper_edge::dto::relationship_dto::RelationshipDto;

pub fn relationship_to_dto(rel: &Relationship<String, String, String>) -> RelationshipDto {
    RelationshipDto {
        node_1: rel.node_1.clone(),
        node_2: rel.node_2.clone(),
        directed: rel.directed,
        edge_properties: rel.edge_properties.clone(),
    }
}

pub fn relationship_from_dto(dto: &RelationshipDto) -> Relationship<String, String, String> {
    Relationship {
        node_1: dto.node_1.clone(),
        node_2: dto.node_2.clone(),
        directed: dto.directed,
        edge_properties: dto.edge_properties.clone(),
    }
}