use super::get_canonical_paths;
use super::get_files;
use super::get_forest;
use super::get_full_resource_folder;
use crate::model;

pub fn main(
    configuration: model::Configuration,
    resource_type: model::TypeAlias,
) -> model::Result<model::FileIndex> {
    let full_resource_folder = get_full_resource_folder::main(&configuration)?;
    let canonical_paths = get_canonical_paths::main(&full_resource_folder)?;
    let files = get_files::main(&full_resource_folder, canonical_paths);
    let forest = get_forest::main(files);
    Ok(model::FileIndex {
        resource_type: resource_type.identifier,
        forest,
    })
}
