use super::get_files;
use super::get_forest;
use super::get_full_resource_folder;
use super::get_paths;
use crate::model;

pub fn main(
    configuration: model::Configuration,
    resource_type: model::ResourceType,
) -> model::Result<model::FileIndex> {
    let full_resource_folder = get_full_resource_folder::main(&configuration)?;
    let paths = get_paths::main(&configuration, &full_resource_folder)?;
    let files = get_files::main(&full_resource_folder, paths);
    let forest = get_forest::main(files);
    Ok(model::FileIndex {
        resource_type: resource_type.identifier,
        forest,
    })
}
