use super::get_base_folder;
use super::get_files;
use super::get_forest;
use super::get_paths;
use crate::model;

pub fn main(
    configuration: model::Configuration,
    resource_type: model::ResourceType,
) -> model::Result<model::FileIndex> {
    let base_folder = get_base_folder::main()?;
    let paths = get_paths::main(&configuration, &base_folder)?;
    let files = get_files::main(&base_folder, paths);
    let forest = get_forest::main(&configuration, files)?;
    Ok(model::FileIndex {
        resource_type: resource_type.identifier,
        forest,
    })
}
