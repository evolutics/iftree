use super::get_field_term;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    paths
        .into_iter()
        .map(|path| get_file(configuration, resource_structure, base_folder, path))
        .collect()
}

fn get_file(
    configuration: &model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::Result<model::File> {
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = absolute_path.to_string_lossy();

    let resource_term = match resource_structure {
        model::ResourceTypeStructure::Unit => model::ResourceTerm::Unit,

        model::ResourceTypeStructure::TypeAlias => {
            model::ResourceTerm::TypeAlias(get_field_term::main(
                configuration,
                absolute_path.as_ref(),
                model::FieldIdentifier::Anonymous,
            )?)
        }

        model::ResourceTypeStructure::NamedFields(names) => model::ResourceTerm::NamedFields(
            names
                .iter()
                .map(|name| {
                    let term = get_field_term::main(
                        configuration,
                        absolute_path.as_ref(),
                        model::FieldIdentifier::Named(name.clone()),
                    )?;
                    Ok((name.clone(), term))
                })
                .collect::<model::Result<_>>()?,
        ),

        model::ResourceTypeStructure::TupleFields(length) => model::ResourceTerm::TupleFields(
            (0..*length)
                .map(|index| {
                    get_field_term::main(
                        configuration,
                        absolute_path.as_ref(),
                        model::FieldIdentifier::Indexed(index),
                    )
                })
                .collect::<model::Result<_>>()?,
        ),
    };

    Ok(model::File {
        relative_path,
        resource_term,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_unit() {
        let actual = main(
            &model::stubs::configuration(),
            &model::ResourceTypeStructure::Unit,
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                resource_term: model::ResourceTerm::Unit,
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::Unit,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_type_alias() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &model::ResourceTypeStructure::TypeAlias,
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                    include_str!("/resources/world/physical_constants.json")
                }),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::TypeAlias(quote::quote! {
                    include_str!("/resources/configuration/menu.json")
                }),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_named_fields() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Named(String::from("content")),
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &model::ResourceTypeStructure::NamedFields(vec![String::from("content")]),
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                resource_term: model::ResourceTerm::NamedFields(vec![(
                    String::from("content"),
                    quote::quote! {
                        include_str!("/resources/world/physical_constants.json")
                    },
                )]),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::NamedFields(vec![(
                    String::from("content"),
                    quote::quote! {
                        include_str!("/resources/configuration/menu.json")
                    },
                )]),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_tuple_fields() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Indexed(0),
                    String::from("include_str!({{absolute_path}})"),
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            &model::ResourceTypeStructure::TupleFields(1),
            path::Path::new("/resources"),
            vec![
                path::PathBuf::from("world/physical_constants.json"),
                path::PathBuf::from("configuration/menu.json"),
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            model::File {
                relative_path: path::PathBuf::from("world/physical_constants.json"),
                resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                    include_str!("/resources/world/physical_constants.json")
                }]),
            },
            model::File {
                relative_path: path::PathBuf::from("configuration/menu.json"),
                resource_term: model::ResourceTerm::TupleFields(vec![quote::quote! {
                    include_str!("/resources/configuration/menu.json")
                }]),
            },
        ];
        assert_eq!(actual, expected);
    }
}
