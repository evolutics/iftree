use super::get_canonical_paths;
use super::get_files;
use super::get_forest;
use crate::model;

pub fn main(resource_type: model::TypeAlias) -> model::FileIndex {
    let canonical_paths = get_canonical_paths::main();
    let files = get_files::main(canonical_paths);
    let forest = get_forest::main(files);
    model::FileIndex {
        resource_type: resource_type.identifier.to_string(),
        forest,
    }
}
