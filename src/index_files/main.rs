use super::get_forest;
use super::get_paths;
use crate::model;

pub fn main(resource_type: model::TypeAlias) -> model::FileIndex {
    let canonical_paths = get_paths::main();
    let forest = get_forest::main(canonical_paths);
    model::FileIndex {
        resource_type: resource_type.identifier.to_string(),
        forest,
    }
}
