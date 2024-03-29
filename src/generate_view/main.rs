use super::get_forest;
use super::get_visitors;
use crate::model;

pub fn main(
    configuration: model::Configuration,
    type_: model::Type<()>,
    paths: Vec<model::Path>,
) -> model::Result<model::View> {
    let visitors = get_visitors::main(configuration.template, type_.structure)?;
    let forest = get_forest::main(paths)?;
    Ok(model::View {
        type_: type_.name,
        visitors,
        forest,
        debug: configuration.debug,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            model::Configuration {
                template: model::Template::Default {
                    initializer: Some(syn::parse_str("abc").unwrap()),
                    identifiers: true,
                },
                debug: true,
                ..model::stubs::configuration()
            },
            model::Type {
                name: quote::format_ident!("Asset"),
                ..model::stubs::type_()
            },
            vec![model::Path {
                relative: vec!["b".into()],
                absolute: "/a/b".into(),
            }],
        );

        let actual = actual.unwrap();
        let expected = model::View {
            type_: quote::format_ident!("Asset"),
            visitors: vec![
                model::Visitor::Array(model::Initializer::Macro(syn::parse_str("abc").unwrap())),
                model::Visitor::Identifiers,
            ],
            forest: [(
                "b".into(),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#B"),
                    index: 0,
                    relative_path: "b".into(),
                    absolute_path: "/a/b".into(),
                }),
            )]
            .into_iter()
            .collect(),
            debug: true,
        };
        assert_eq!(actual, expected);
    }
}
