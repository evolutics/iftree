use super::get_base_folder;
use super::get_files;
use super::get_forest;
use super::get_paths;
use crate::model;
use std::env;

pub fn main(
    configuration: model::Configuration,
    resource_type: model::ResourceType,
) -> model::Result<model::FileIndex> {
    let base_folder = get_base_folder::main(&|name| env::var(name))?;
    let paths = get_paths::main(&configuration, &base_folder)?;
    let files = get_files::main(&base_folder, paths);
    let forest = get_forest::main(&configuration, files)?;
    Ok(model::FileIndex {
        resource_type: resource_type.identifier,
        forest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path;

    #[test]
    fn indexes() {
        let actual = main(
            model::Configuration {
                resource_paths: String::from("examples/resources/credits.md"),
                ..model::stubs::configuration()
            },
            model::ResourceType {
                identifier: String::from("Resource"),
                ..model::stubs::resource_type()
            },
        );

        let actual = actual.unwrap();
        let mut resources = model::FileForest::new();
        resources.insert(
            String::from("r#CREDITS_MD"),
            model::FileTree::File(model::File {
                relative_path: path::PathBuf::from("examples/resources/credits.md"),
                absolute_path: fs::canonicalize("examples/resources/credits.md").unwrap(),
            }),
        );
        let mut examples = model::FileForest::new();
        examples.insert(
            String::from("r#resources"),
            model::FileTree::Folder(resources),
        );
        let mut forest = model::FileForest::new();
        forest.insert(
            String::from("r#examples"),
            model::FileTree::Folder(examples),
        );
        let expected = model::FileIndex {
            resource_type: String::from("Resource"),
            forest,
        };
        assert_eq!(actual, expected);
    }
}
