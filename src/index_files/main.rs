use crate::model;

pub fn main(resource_type: model::TypeAlias) -> model::FileIndex {
    model::FileIndex {
        resource_type: resource_type.identifier.to_string(),
    }
}
