use crate::hyper_edge::dto::property_dto::{PropertyDto, PropertyTypeDto};
use crate::hyper_edge::entity::h_edge::simple_h_edge::{Property, PropertyType};

pub fn property_to_dto(prop: &Property<String, String>) -> PropertyDto {
    PropertyDto {
        key: prop.key.clone(),
        value: prop.value.clone(),
        p_type: match prop.p_type {
            PropertyType::Simple => PropertyTypeDto::Simple,
            PropertyType::Main => PropertyTypeDto::Main,
            PropertyType::Structure => PropertyTypeDto::Structure,
            PropertyType::ExtraInfo => PropertyTypeDto::ExtraInfo,
        },
    }
}

pub fn property_from_dto(dto: &PropertyDto) -> Property<String, String> {
    Property {
        key: dto.key.clone(),
        value: dto.value.clone(),
        p_type: match dto.p_type {
            PropertyTypeDto::Simple => PropertyType::Simple,
            PropertyTypeDto::Main => PropertyType::Main,
            PropertyTypeDto::Structure => PropertyType::Structure,
            PropertyTypeDto::ExtraInfo => PropertyType::ExtraInfo,
        },
    }
}

pub fn properties_to_dto(props: &[Property<String, String>]) -> Vec<PropertyDto> {
    props.iter().map(property_to_dto).collect()
}

pub fn properties_from_dto(dtos: &[PropertyDto]) -> Vec<Property<String, String>> {
    dtos.iter().map(property_from_dto).collect()
}