use super::get_default_initializer;
use crate::model;
use std::iter;

pub fn main(
    template: model::Template,
    structure: model::TypeStructure<()>,
) -> model::Result<Vec<model::Visitor>> {
    Ok(match template {
        model::Template::Default {
            initializer,
            identifiers,
        } => {
            let initializer = match initializer {
                None => model::Initializer::Default(get_default_initializer::main(structure)?),
                Some(macro_) => model::Initializer::Macro(macro_),
            };

            iter::once(model::Visitor::Array(initializer))
                .chain(iter::once(model::Visitor::Identifiers).filter(|_| identifiers))
                .collect()
        }

        model::Template::Visitors(visitors) => {
            visitors.into_iter().map(model::Visitor::Custom).collect()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod handles_default {
        use super::*;

        #[test]
        fn handles_without_initializer() {
            let actual = main(
                model::Template::Default {
                    initializer: None,
                    identifiers: false,
                },
                model::TypeStructure::Unit,
            );

            let actual = actual.unwrap();
            let expected = vec![model::Visitor::Array(model::Initializer::Default(
                model::TypeStructure::Unit,
            ))];
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_with_initializer() {
            let actual = main(
                model::Template::Default {
                    initializer: Some(syn::parse_str("abc").unwrap()),
                    identifiers: false,
                },
                model::stubs::type_structure(),
            );

            let actual = actual.unwrap();
            let expected = vec![model::Visitor::Array(model::Initializer::Macro(
                syn::parse_str("abc").unwrap(),
            ))];
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_without_identifiers() {
            let actual = main(
                model::Template::Default {
                    initializer: Some(syn::parse_str("abc").unwrap()),
                    identifiers: false,
                },
                model::stubs::type_structure(),
            );

            let actual = actual.unwrap();
            let expected = vec![model::Visitor::Array(model::Initializer::Macro(
                syn::parse_str("abc").unwrap(),
            ))];
            assert_eq!(actual, expected);
        }

        #[test]
        fn handles_with_identifiers() {
            let actual = main(
                model::Template::Default {
                    initializer: Some(syn::parse_str("abc").unwrap()),
                    identifiers: true,
                },
                model::stubs::type_structure(),
            );

            let actual = actual.unwrap();
            let expected = vec![
                model::Visitor::Array(model::Initializer::Macro(syn::parse_str("abc").unwrap())),
                model::Visitor::Identifiers,
            ];
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn handles_visitors() {
        let actual = main(
            model::Template::Visitors(vec![model::CustomVisitor {
                visit_base: Some(syn::parse_str("visit_base").unwrap()),
                visit_folder: Some(syn::parse_str("visit_folder").unwrap()),
                visit_file: syn::parse_str("visit_file").unwrap(),
            }]),
            model::stubs::type_structure(),
        );

        let actual = actual.unwrap();
        let expected = vec![model::Visitor::Custom(model::CustomVisitor {
            visit_base: Some(syn::parse_str("visit_base").unwrap()),
            visit_folder: Some(syn::parse_str("visit_folder").unwrap()),
            visit_file: syn::parse_str("visit_file").unwrap(),
        })];
        assert_eq!(actual, expected);
    }
}
