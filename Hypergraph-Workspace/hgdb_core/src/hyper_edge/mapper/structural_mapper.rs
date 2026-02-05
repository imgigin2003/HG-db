use crate::hyper_edge::entity::structure::structure::{StructuralProperty, Traverse};
use crate::hyper_edge::dto::structural_dto::{StructuralPropertyDto, TraverseDto};

pub fn structural_to_dto(sp: &StructuralProperty) -> StructuralPropertyDto {
    StructuralPropertyDto {
        address: sp.address.clone(),
        layer_index: sp.layer_index,
    }
}

pub fn structural_from_dto(dto: &StructuralPropertyDto) -> StructuralProperty {
    StructuralProperty {
        address: dto.address.clone(),
        layer_index: dto.layer_index,
    }
}

pub fn structurals_to_dto(sps: &[StructuralProperty]) -> Vec<StructuralPropertyDto> {
    sps.iter().map(structural_to_dto).collect()
}

pub fn structurals_from_dto(dtos: &[StructuralPropertyDto]) -> Vec<StructuralProperty> {
    dtos.iter().map(structural_from_dto).collect()
}

pub fn traverse_to_dto(t: &Traverse) -> TraverseDto {
    TraverseDto {
        path: t.path.clone(),
        weight: None,         
    }
}

pub fn traverse_from_dto(dto: &TraverseDto) -> Traverse {
    Traverse {
        path: dto.path.clone(),
    }
}