use super::get_initializer;
use crate::model;
use std::iter;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    structure: model::TypeStructure<()>,
) -> model::Result<vec::Vec<model::Visitor>> {
    let initializer = get_initializer::main(configuration, structure)?;
    Ok(iter::once(model::Visitor::Array(initializer))
        .chain(iter::once(model::Visitor::Identifiers).filter(|_| configuration.identifiers))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_without_identifiers() {
        let actual = main(
            &model::Configuration {
                initializer: Some(syn::parse_str("abc").unwrap()),
                identifiers: false,
                ..model::stubs::configuration()
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
            &model::Configuration {
                initializer: Some(syn::parse_str("abc").unwrap()),
                identifiers: true,
                ..model::stubs::configuration()
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
