use crate::data;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    structure: model::TypeStructure<()>,
) -> model::Result<model::Initializer> {
    Ok(match &configuration.initializer {
        None => model::Initializer::Default(get_templates(structure)?),
        Some(macro_) => model::Initializer::Macro(macro_.clone()),
    })
}

fn get_templates(
    structure: model::TypeStructure<()>,
) -> model::Result<model::TypeStructure<model::Template>> {
    match structure {
        model::TypeStructure::Unit => Ok(model::TypeStructure::Unit),

        model::TypeStructure::TypeAlias(_) => Err(model::Error::NoInitializer),

        model::TypeStructure::NamedFields(fields) => Ok(model::TypeStructure::NamedFields(
            fields
                .into_iter()
                .map(|(field, _)| {
                    let template = get_template(&field)?;
                    Ok((field, template))
                })
                .collect::<model::Result<_>>()?,
        )),

        model::TypeStructure::TupleFields(unary_length) => {
            if unary_length.is_empty() {
                Ok(model::TypeStructure::TupleFields(vec![]))
            } else {
                Err(model::Error::NoInitializer)
            }
        }
    }
}

fn get_template(field: &str) -> model::Result<model::Template> {
    match data::STANDARD_FIELD_TEMPLATES_ORDERED.binary_search_by_key(&field, |entry| entry.0) {
        Err(_) => Err(model::Error::NonstandardField {
            field: String::from(field),
        }),
        Ok(index) => Ok(data::STANDARD_FIELD_TEMPLATES_ORDERED[index].1.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod given_no_initializer {
        use super::*;

        #[test]
        fn handles_unit() {
            let actual = main(
                &model::Configuration {
                    initializer: None,
                    ..model::stubs::configuration()
                },
                model::TypeStructure::Unit,
            );

            let actual = actual.unwrap();
            let expected = model::Initializer::Default(model::TypeStructure::Unit);
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_type_alias() {
            let actual = main(
                &model::Configuration {
                    initializer: None,
                    ..model::stubs::configuration()
                },
                model::TypeStructure::TypeAlias(()),
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::NoInitializer;
            assert_eq!(actual, expected);
        }

        #[cfg(test)]
        mod handles_named_fields {
            use super::*;

            #[test]
            fn given_standard_fields_only_it_handles() {
                let actual = main(
                    &model::Configuration {
                        initializer: None,
                        ..model::stubs::configuration()
                    },
                    model::TypeStructure::NamedFields(vec![
                        (String::from("relative_path"), ()),
                        (String::from("contents_str"), ()),
                    ]),
                );

                let actual = actual.unwrap();
                let expected =
                    model::Initializer::Default(model::TypeStructure::NamedFields(vec![
                        (String::from("relative_path"), model::Template::RelativePath),
                        (String::from("contents_str"), model::Template::ContentsStr),
                    ]));
                assert_eq!(actual, expected);
            }

            #[test]
            fn given_nonstandard_field_it_errs() {
                let actual = main(
                    &model::Configuration {
                        initializer: None,
                        ..model::stubs::configuration()
                    },
                    model::TypeStructure::NamedFields(vec![
                        (String::from("relative_path"), ()),
                        (String::from("abc"), ()),
                    ]),
                );

                let actual = actual.unwrap_err();
                let expected = model::Error::NonstandardField {
                    field: String::from("abc"),
                };
                assert_eq!(actual, expected);
            }
        }

        #[cfg(test)]
        mod handles_tuple_fields {
            use super::*;

            #[test]
            fn given_no_fields_it_handles() {
                let actual = main(
                    &model::Configuration {
                        initializer: None,
                        ..model::stubs::configuration()
                    },
                    model::TypeStructure::TupleFields(vec![]),
                );

                let actual = actual.unwrap();
                let expected =
                    model::Initializer::Default(model::TypeStructure::TupleFields(vec![]));
                assert_eq!(actual, expected);
            }

            #[test]
            fn given_fields_it_errs() {
                let actual = main(
                    &model::Configuration {
                        initializer: None,
                        ..model::stubs::configuration()
                    },
                    model::TypeStructure::TupleFields(vec![()]),
                );

                let actual = actual.unwrap_err();
                let expected = model::Error::NoInitializer;
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn given_initializer_it_handles() {
        let actual = main(
            &model::Configuration {
                initializer: Some(String::from("abc")),
                ..model::stubs::configuration()
            },
            model::stubs::type_structure(),
        );

        let actual = actual.unwrap();
        let expected = model::Initializer::Macro(String::from("abc"));
        assert_eq!(actual, expected);
    }
}
