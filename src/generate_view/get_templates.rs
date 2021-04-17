use crate::data;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    type_: model::Type<()>,
) -> model::Result<model::Type<model::Template>> {
    Ok(model::Type {
        identifier: type_.identifier,

        structure: match type_.structure {
            model::TypeStructure::Unit => model::TypeStructure::Unit,

            model::TypeStructure::TypeAlias(_) => model::TypeStructure::TypeAlias(get_template(
                configuration,
                model::Field::Anonymous,
            )?),

            model::TypeStructure::NamedFields(names) => model::TypeStructure::NamedFields(
                names
                    .into_iter()
                    .map(|(name, _)| {
                        Ok((
                            name.clone(),
                            get_template(configuration, model::Field::Named(name))?,
                        ))
                    })
                    .collect::<model::Result<_>>()?,
            ),

            model::TypeStructure::TupleFields(unary_length) => model::TypeStructure::TupleFields(
                unary_length
                    .iter()
                    .enumerate()
                    .map(|(index, _)| get_template(configuration, model::Field::Indexed(index)))
                    .collect::<model::Result<_>>()?,
            ),
        },
    })
}

fn get_template(
    configuration: &model::Configuration,
    field: model::Field,
) -> model::Result<model::Template> {
    match configuration.field_templates.get(&field) {
        None => {
            let name = String::from(field.clone());
            match data::PREDEFINED_TEMPLATES_ORDERED.binary_search_by(|entry| entry.0.cmp(&name)) {
                Err(_) => Err(model::Error::MissingFieldTemplate(field)),
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
            model::Type {
                structure: model::TypeStructure::TypeAlias(()),
                ..model::stubs::type_()
            },
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::MissingFieldTemplate(model::Field::Anonymous);
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_no_configured_field_template_it_defaults_to_predefined() {
        let actual = main(
            &model::Configuration {
                field_templates: model::FieldTemplates::new(),
                ..model::stubs::configuration()
            },
            model::Type {
                identifier: quote::format_ident!("Asset"),
                structure: model::TypeStructure::NamedFields(vec![(String::from("content"), ())]),
            },
        );

        let actual = actual.unwrap();
        let expected = model::Type {
            identifier: quote::format_ident!("Asset"),
            structure: model::TypeStructure::NamedFields(vec![(
                String::from("content"),
                model::Template::Content,
            )]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_configured_field_template_it_handles_it() {
        let actual = main(
            &model::Configuration {
                field_templates: vec![(
                    model::Field::Named(String::from("content")),
                    model::Template::RawContent,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            model::Type {
                identifier: quote::format_ident!("Asset"),
                structure: model::TypeStructure::NamedFields(vec![(String::from("content"), ())]),
            },
        );

        let actual = actual.unwrap();
        let expected = model::Type {
            identifier: quote::format_ident!("Asset"),
            structure: model::TypeStructure::NamedFields(vec![(
                String::from("content"),
                model::Template::RawContent,
            )]),
        };
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod type_cases {
        use super::*;

        #[test]
        fn handles_unit() {
            let actual = main(
                &model::stubs::configuration(),
                model::Type {
                    identifier: quote::format_ident!("MyUnit"),
                    structure: model::TypeStructure::Unit,
                },
            );

            let actual = actual.unwrap();
            let expected = model::Type {
                identifier: quote::format_ident!("MyUnit"),
                structure: model::TypeStructure::Unit,
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_type_alias() {
            let actual = main(
                &model::Configuration {
                    field_templates: vec![(model::Field::Anonymous, model::Template::Content)]
                        .into_iter()
                        .collect(),
                    ..model::stubs::configuration()
                },
                model::Type {
                    identifier: quote::format_ident!("MyTypeAlias"),
                    structure: model::TypeStructure::TypeAlias(()),
                },
            );

            let actual = actual.unwrap();
            let expected = model::Type {
                identifier: quote::format_ident!("MyTypeAlias"),
                structure: model::TypeStructure::TypeAlias(model::Template::Content),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_named_fields() {
            let actual = main(
                &model::Configuration {
                    field_templates: vec![(
                        model::Field::Named(String::from("my_content")),
                        model::Template::RawContent,
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::configuration()
                },
                model::Type {
                    identifier: quote::format_ident!("MyNamedFields"),
                    structure: model::TypeStructure::NamedFields(vec![(
                        String::from("my_content"),
                        (),
                    )]),
                },
            );

            let actual = actual.unwrap();
            let expected = model::Type {
                identifier: quote::format_ident!("MyNamedFields"),
                structure: model::TypeStructure::NamedFields(vec![(
                    String::from("my_content"),
                    model::Template::RawContent,
                )]),
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_tuple_fields() {
            let actual = main(
                &model::Configuration {
                    field_templates: vec![(
                        model::Field::Indexed(0),
                        model::Template::RelativePath,
                    )]
                    .into_iter()
                    .collect(),
                    ..model::stubs::configuration()
                },
                model::Type {
                    identifier: quote::format_ident!("MyTupleFields"),
                    structure: model::TypeStructure::TupleFields(vec![()]),
                },
            );

            let actual = actual.unwrap();
            let expected = model::Type {
                identifier: quote::format_ident!("MyTupleFields"),
                structure: model::TypeStructure::TupleFields(vec![model::Template::RelativePath]),
            };
            assert_eq!(actual, expected);
        }
    }
}
