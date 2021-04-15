use crate::data;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    resource_type: model::ResourceType<()>,
) -> model::Result<model::ResourceType<model::Template>> {
    Ok(model::ResourceType {
        identifier: resource_type.identifier,

        structure: match resource_type.structure {
            model::ResourceStructure::Unit => model::ResourceStructure::Unit,

            model::ResourceStructure::TypeAlias(_) => model::ResourceStructure::TypeAlias(
                get_template(configuration, model::FieldIdentifier::Anonymous)?,
            ),

            model::ResourceStructure::NamedFields(names) => model::ResourceStructure::NamedFields(
                names
                    .iter()
                    .map(|(name, _)| {
                        Ok((
                            name.clone(),
                            get_template(
                                configuration,
                                model::FieldIdentifier::Named(String::from(name)),
                            )?,
                        ))
                    })
                    .collect::<model::Result<_>>()?,
            ),

            model::ResourceStructure::TupleFields(structure) => {
                model::ResourceStructure::TupleFields(
                    structure
                        .iter()
                        .enumerate()
                        .map(|(index, _)| {
                            get_template(configuration, model::FieldIdentifier::Indexed(index))
                        })
                        .collect::<model::Result<_>>()?,
                )
            }
        },
    })
}

fn get_template(
    configuration: &model::Configuration,
    identifier: model::FieldIdentifier,
) -> model::Result<model::Template> {
    match configuration.field_templates.get(&identifier) {
        None => {
            let name = String::from(identifier.clone());
            match data::PREDEFINED_TEMPLATES_ORDERED.binary_search_by(|entry| entry.0.cmp(&name)) {
                Err(_) => Err(model::Error::MissingFieldTemplate(identifier)),
                Ok(index) => Ok(data::PREDEFINED_TEMPLATES_ORDERED[index].1.clone()),
            }
        }

        Some(template) => Ok(template.clone()),
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
            model::ResourceType {
                structure: model::ResourceStructure::TypeAlias(()),
                ..model::stubs::resource_type()
            },
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::FieldIdentifier::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_configured_field_template_it_defaults_to_predefined() {
        let actual = main(
            &model::Configuration {
                field_templates: model::FieldTemplates::new(),
                ..model::stubs::configuration()
            },
            model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::NamedFields(vec![(
                    String::from("content"),
                    (),
                )]),
            },
        );

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("Resource"),
            structure: model::ResourceStructure::NamedFields(vec![(
                String::from("content"),
                model::Template::Content,
            )]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_configured_field_template_it_gets_it() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::FieldIdentifier::Named(String::from("content")),
                    model::Template::RawContent,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            model::ResourceType {
                identifier: quote::format_ident!("Resource"),
                structure: model::ResourceStructure::NamedFields(vec![(
                    String::from("content"),
                    (),
                )]),
            },
        );

        let actual = actual.unwrap();
        let expected = model::ResourceType {
            identifier: quote::format_ident!("Resource"),
            structure: model::ResourceStructure::NamedFields(vec![(
                String::from("content"),
                model::Template::RawContent,
            )]),
        };
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod resource_cases {
        use super::*;

        #[test]
        fn gets_unit() {
            let actual = main(
                &model::stubs::configuration(),
                model::ResourceType {
                    identifier: quote::format_ident!("MyUnit"),
                    structure: model::ResourceStructure::Unit,
                },
            );

            let actual = actual.unwrap();
            let expected = model::ResourceType {
                identifier: quote::format_ident!("MyUnit"),
                structure: model::ResourceStructure::Unit,
            };
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
                model::ResourceType {
                    identifier: quote::format_ident!("MyTypeAlias"),
                    structure: model::ResourceStructure::TypeAlias(()),
                },
            );

            let actual = actual.unwrap();
            let expected = model::ResourceType {
                identifier: quote::format_ident!("MyTypeAlias"),
                structure: model::ResourceStructure::TypeAlias(model::Template::Content),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_named_fields() {
            let actual = main(
                &model::Configuration {
                    field_templates: vec![(
                        model::FieldIdentifier::Named(String::from("my_content")),
                        model::Template::RawContent,
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::configuration()
                },
                model::ResourceType {
                    identifier: quote::format_ident!("MyNamedFields"),
                    structure: model::ResourceStructure::NamedFields(vec![(
                        String::from("my_content"),
                        (),
                    )]),
                },
            );

            let actual = actual.unwrap();
            let expected = model::ResourceType {
                identifier: quote::format_ident!("MyNamedFields"),
                structure: model::ResourceStructure::NamedFields(vec![(
                    String::from("my_content"),
                    model::Template::RawContent,
                )]),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn gets_tuple_fields() {
            let actual = main(
                &model::Configuration {
                    field_templates: vec![(
                        model::FieldIdentifier::Indexed(0),
                        model::Template::RelativePath,
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::configuration()
                },
                model::ResourceType {
                    identifier: quote::format_ident!("MyTupleFields"),
                    structure: model::ResourceStructure::TupleFields(vec![()]),
                },
            );

            let actual = actual.unwrap();
            let expected = model::ResourceType {
                identifier: quote::format_ident!("MyTupleFields"),
                structure: model::ResourceStructure::TupleFields(vec![
                    model::Template::RelativePath,
                ]),
            };
            assert_eq!(actual, expected);
        }
    }
}
