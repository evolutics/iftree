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
    let base_folder = get_base_folder::main(&configuration, &|name| env::var(name))?;
    let paths = get_paths::main(&configuration, &base_folder)?;
    let files = get_files::main(
        &configuration,
        &resource_type.structure,
        &base_folder,
        paths,
    )?;
    let forest = get_forest::main(&configuration, files)?;
    Ok(model::FileIndex {
        resource_type: resource_type.identifier,
        forest,
        generate_array: true,
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
                base_folder_environment_variable: String::from("CARGO_MANIFEST_DIR"),
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            model::ResourceType {
                identifier: String::from("Resource"),
                structure: model::ResourceTypeStructure::TypeAlias,
            },
        );

        let actual = actual.unwrap();
        let absolute_path = fs::canonicalize("examples/resources/credits.md")
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let expected = model::FileIndex {
            resource_type: String::from("Resource"),
            forest: vec![(
                String::from("r#examples"),
                model::FileTree::Folder(
                    vec![(
                        String::from("r#resources"),
                        model::FileTree::Folder(
                            vec![(
                                String::from("r#CREDITS_MD"),
                                model::FileTree::File(model::File {
                                    relative_path: path::PathBuf::from(
                                        "examples/resources/credits.md",
                                    ),
                                    resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                                        include_str!(#absolute_path)
                                    }),
                                }),
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
            generate_array: true,
        };
        assert_eq!(actual, expected);
    }
}
