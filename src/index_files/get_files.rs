use super::render_field_template;
use super::try_map_abstract_resource;
use crate::data;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    let annotated_resource = annotate_resource(configuration, resource_structure)?;
    Ok(paths
        .into_iter()
        .map(|path| get_file(&annotated_resource, base_folder, path))
        .collect())
}

fn annotate_resource<'a>(
    configuration: &'a model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
) -> model::Result<model::AbstractResource<&'a model::Template>> {
    impl<'a> try_map_abstract_resource::TryMap for &'a model::Configuration {
        type Input = ();
        type Output = &'a model::Template;

        fn map_unit(&self) -> model::Result<()> {
            Ok(())
        }

        fn map_type_alias(&self, _annotation: &Self::Input) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Anonymous)
        }

        fn map_named_field(
            &self,
            name: &str,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Named(String::from(name)))
        }

        fn map_tuple_field(
            &self,
            index: usize,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_template(self, model::FieldIdentifier::Indexed(index))
        }
    }

    try_map_abstract_resource::main(&configuration, resource_structure)
}

fn get_template(
    configuration: &model::Configuration,
    identifier: model::FieldIdentifier,
) -> model::Result<&model::Template> {
    match configuration.field_templates.get(&identifier) {
        None => {
            let name = String::from(identifier.clone());
            match data::PREDEFINED_TEMPLATES_ORDERED.binary_search_by(|entry| entry.0.cmp(&name)) {
                Err(_) => Err(model::Error::MissingFieldTemplate(identifier)),
                Ok(index) => Ok(&data::PREDEFINED_TEMPLATES_ORDERED[index].1),
            }
        }

        Some(template) => Ok(template),
    }
}

fn get_file(
    annotated_resource: &model::AbstractResource<&model::Template>,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::File {
    let raw_relative_path = &relative_path.to_string_lossy();
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = &absolute_path.to_string_lossy();
    let context = render_field_template::Context {
        relative_path: raw_relative_path,
        absolute_path,
    };

    let resource_term = match annotated_resource {
        model::AbstractResource::Unit => model::AbstractResource::Unit,

        model::AbstractResource::TypeAlias(template) => {
            model::AbstractResource::TypeAlias(render_field_template::main(template, &context))
        }

        model::AbstractResource::NamedFields(named_templates) => {
            model::AbstractResource::NamedFields(
                named_templates
                    .iter()
                    .map(|(name, template)| {
                        (
                            name.clone(),
                            render_field_template::main(template, &context),
                        )
                    })
                    .collect(),
            )
        }

        model::AbstractResource::TupleFields(templates) => model::AbstractResource::TupleFields(
            templates
                .iter()
                .map(|template| render_field_template::main(template, &context))
                .collect(),
        ),
    };

    model::File {
        relative_path,
        resource_term,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_no_field_template_at_all_it_errs() {
        let actual = main(
            &model::Configuration {
                field_templates: model::FieldTemplates::new(),
                ..model::stubs::configuration()
            },
            &model::ResourceTypeStructure::TypeAlias(()),
            path::Path::new("/foo"),
            vec![],
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::FieldIdentifier::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_custom_field_template_it_defaults_to_predefined() {
        let actual = main(
            &model::Configuration {
                field_templates: model::FieldTemplates::new(),
                ..model::stubs::configuration()
            },
            &model::ResourceTypeStructure::NamedFields(vec![(String::from("raw_content"), ())]),
            path::Path::new("/resources"),
            vec![path::PathBuf::from("credits.md")],
        );

        let actual = actual.unwrap();
        let expected = vec![model::File {
            relative_path: path::PathBuf::from("credits.md"),
            resource_term: model::ResourceTerm::NamedFields(vec![(
                String::from("raw_content"),
                quote::quote! {
                    include_bytes!("/resources/credits.md")
                },
            )]),
        }];
        assert_eq!(actual, expected);
    }

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
                    model::Template::Content,
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
                    model::Template::Content,
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
                    model::Template::Content,
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
            field_templates: vec![model::Template::RelativePath, model::Template::AbsolutePath]
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
