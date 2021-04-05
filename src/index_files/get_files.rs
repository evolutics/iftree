use super::get_field_term;
use super::render_field_template;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    let annotated_resource = annotate_resource(resource_structure);
    paths
        .into_iter()
        .map(|path| get_file(configuration, &annotated_resource, base_folder, path))
        .collect()
}

fn annotate_resource(
    resource_structure: &model::ResourceTypeStructure,
) -> model::AbstractResource<model::FieldIdentifier> {
    match resource_structure {
        model::ResourceTypeStructure::Unit => model::AbstractResource::Unit,

        model::ResourceTypeStructure::TypeAlias(_) => {
            model::AbstractResource::TypeAlias(model::FieldIdentifier::Anonymous)
        }

        model::ResourceTypeStructure::NamedFields(names) => model::AbstractResource::NamedFields(
            names
                .iter()
                .map(|(name, _)| (name.clone(), model::FieldIdentifier::Named(name.clone())))
                .collect(),
        ),

        model::ResourceTypeStructure::TupleFields(structure) => {
            model::AbstractResource::TupleFields(
                structure
                    .iter()
                    .enumerate()
                    .map(|(index, _)| model::FieldIdentifier::Indexed(index))
                    .collect(),
            )
        }
    }
}

fn get_file(
    configuration: &model::Configuration,
    annotated_resource: &model::AbstractResource<model::FieldIdentifier>,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::Result<model::File> {
    let raw_relative_path = &relative_path.to_string_lossy();
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = &absolute_path.to_string_lossy();
    let context = render_field_template::Context {
        relative_path: raw_relative_path,
        absolute_path,
    };

    let resource_term = match annotated_resource {
        model::AbstractResource::Unit => model::ResourceTerm::Unit,

        model::AbstractResource::TypeAlias(identifier) => model::ResourceTerm::TypeAlias(
            get_field_term::main(configuration, &context, identifier)?,
        ),

        model::AbstractResource::NamedFields(named_identifiers) => {
            model::ResourceTerm::NamedFields(
                named_identifiers
                    .iter()
                    .map(|(name, identifier)| {
                        let term = get_field_term::main(configuration, &context, identifier)?;
                        Ok((name.clone(), term))
                    })
                    .collect::<model::Result<_>>()?,
            )
        }

        model::AbstractResource::TupleFields(identifiers) => model::ResourceTerm::TupleFields(
            identifiers
                .iter()
                .map(|identifier| get_field_term::main(configuration, &context, identifier))
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
    fn gets_type_unit() {
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
            &model::ResourceTypeStructure::TypeAlias(()),
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
    fn gets_type_named_fields() {
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
            &model::ResourceTypeStructure::NamedFields(vec![(String::from("content"), ())]),
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
    fn gets_type_tuple_fields() {
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
            &model::ResourceTypeStructure::TupleFields(vec![()]),
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

    #[test]
    fn gets_template_context() {
        let configuration = model::Configuration {
            field_templates: vec![
                String::from("{{relative_path}}"),
                String::from("{{absolute_path}}"),
            ]
            .into_iter()
            .enumerate()
            .map(|(index, template)| (model::FieldIdentifier::Indexed(index), template))
            .collect(),
            ..model::stubs::configuration()
        };

        let actual = main(
            &configuration,
            &model::ResourceTypeStructure::TupleFields(
                configuration.field_templates.iter().map(|_| ()).collect(),
            ),
            path::Path::new("/resources"),
            vec![path::PathBuf::from("credits.md")],
        );

        let actual = actual.unwrap();
        let expected = vec![model::File {
            relative_path: path::PathBuf::from("credits.md"),
            resource_term: model::ResourceTerm::TupleFields(vec![
                quote::quote! {
                    "credits.md"
                },
                quote::quote! {
                    "/resources/credits.md"
                },
            ]),
        }];
        assert_eq!(actual, expected);
    }
}
