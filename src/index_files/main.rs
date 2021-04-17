use super::get_base_folder;
use super::get_files;
use super::get_forest;
use super::get_paths;
use super::get_templates;
use crate::model;
use std::env;

pub fn main(
    configuration: model::Configuration,
    resource_type: model::ResourceType<()>,
) -> model::Result<model::FileIndex> {
    let resource_type = get_templates::main(&configuration, resource_type)?;
    let base_folder = get_base_folder::main(&configuration, &|name| env::var(name))?;
    let paths = get_paths::main(&configuration, &base_folder)?;
    let files = get_files::main(&base_folder, paths)?;
    let forest = get_forest::main(&configuration, &files)?;
    Ok(model::FileIndex {
        resource_type,
        array: files,
        forest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(
            model::Configuration {
                paths: String::from("/examples/resources/credits.md"),
                base_folder: path::PathBuf::new(),
                root_folder_variable: String::from("CARGO_MANIFEST_DIR"),
                module_tree: true,
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    model::Template::Content,
                )]
                .into_iter()
                .collect(),
            },
            model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(()),
            },
        );

        let actual = actual.unwrap();
        let absolute_path = fs::canonicalize("examples/resources/credits.md").unwrap();
        let expected = model::FileIndex {
            resource_type: model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::TypeAlias(model::Template::Content),
            },
            array: vec![model::File {
                relative_path: model::RelativePath::from("examples/resources/credits.md"),
                absolute_path,
            }],
            forest: Some(
                vec![(
                    String::from("r#examples"),
                    model::FileTree::Folder(
                        vec![(
                            String::from("r#resources"),
                            model::FileTree::Folder(
                                vec![(
                                    String::from("r#CREDITS_MD"),
                                    model::FileTree::File { index: 0 },
                                )]
                                .into_iter()
                                .collect(),
                            ),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                )]
                .into_iter()
                .collect(),
            ),
        };
        assert_eq!(actual, expected);
    }
}
