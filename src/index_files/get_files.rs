use super::get_field_term;
use super::render_field_template;
use super::try_map_abstract_resource;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    resource_structure: &model::ResourceTypeStructure,
    base_folder: &path::Path,
    paths: vec::Vec<path::PathBuf>,
) -> model::Result<vec::Vec<model::File>> {
    let annotated_resource = annotate_resource(resource_structure)?;
    paths
        .into_iter()
        .map(|path| get_file(configuration, &annotated_resource, base_folder, path))
        .collect()
}

fn annotate_resource(
    resource_structure: &model::ResourceTypeStructure,
) -> model::Result<model::AbstractResource<model::FieldIdentifier>> {
    struct TryMap;

    impl try_map_abstract_resource::TryMap for TryMap {
        type Input = ();
        type Output = model::FieldIdentifier;

        fn map_unit(&self) -> model::Result<()> {
            Ok(())
        }

        fn map_type_alias(&self, _annotation: &Self::Input) -> model::Result<Self::Output> {
            Ok(model::FieldIdentifier::Anonymous)
        }

        fn map_named_field(
            &self,
            name: &str,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            Ok(model::FieldIdentifier::Named(String::from(name)))
        }

        fn map_tuple_field(
            &self,
            index: usize,
            _annotation: &Self::Input,
        ) -> model::Result<Self::Output> {
            Ok(model::FieldIdentifier::Indexed(index))
        }
    }

    try_map_abstract_resource::main(&TryMap, resource_structure)
}

fn get_file(
    configuration: &model::Configuration,
    annotated_resource: &model::AbstractResource<model::FieldIdentifier>,
    base_folder: &path::Path,
    relative_path: path::PathBuf,
) -> model::Result<model::File> {
    struct TryMap<'a> {
        configuration: &'a model::Configuration,
        context: render_field_template::Context<'a>,
    }

    impl try_map_abstract_resource::TryMap for TryMap<'_> {
        type Input = model::FieldIdentifier;
        type Output = proc_macro2::TokenStream;

        fn map_unit(&self) -> model::Result<()> {
            Ok(())
        }

        fn map_type_alias(&self, identifier: &Self::Input) -> model::Result<Self::Output> {
            get_field_term::main(self.configuration, &self.context, identifier)
        }

        fn map_named_field(
            &self,
            _name: &str,
            identifier: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_field_term::main(self.configuration, &self.context, identifier)
        }

        fn map_tuple_field(
            &self,
            _index: usize,
            identifier: &Self::Input,
        ) -> model::Result<Self::Output> {
            get_field_term::main(self.configuration, &self.context, identifier)
        }
    }

    let raw_relative_path = &relative_path.to_string_lossy();
    let absolute_path = base_folder.join(&relative_path);
    let absolute_path = &absolute_path.to_string_lossy();
    let context = render_field_template::Context {
        relative_path: raw_relative_path,
        absolute_path,
    };

    let try_map = TryMap {
        configuration,
        context,
    };

    let resource_term = try_map_abstract_resource::main(&try_map, annotated_resource)?;

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
