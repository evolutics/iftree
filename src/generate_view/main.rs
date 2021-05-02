use super::get_forest;
use super::get_visitors;
use crate::model;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    type_: model::Type<()>,
    paths: vec::Vec<model::Path>,
) -> model::Result<model::View> {
    let visitors = get_visitors::main(configuration, type_.structure)?;
    let forest = get_forest::main(paths);
    Ok(model::View {
        type_: type_.name,
        visitors,
        forest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles() {
        let actual = main(
            &model::Configuration {
                initializer: Some(String::from("abc")),
                identifiers: true,
                ..model::stubs::configuration()
            },
            model::Type {
                name: quote::format_ident!("Asset"),
                ..model::stubs::type_()
            },
            vec![model::Path {
                relative: model::RelativePath::from("b"),
                absolute: String::from("/a/b"),
            }],
        );

        let actual = actual.unwrap();
        let expected = model::View {
            type_: quote::format_ident!("Asset"),
            visitors: vec![
                model::Visitor::Array(model::Initializer::Macro(String::from("abc"))),
                model::Visitor::Identifiers,
            ],
            forest: vec![(
                String::from('b'),
                model::FileTree::File(model::File {
                    identifier: quote::format_ident!("r#B"),
                    index: 0,
                    relative_path: model::RelativePath::from("b"),
                    absolute_path: String::from("/a/b"),
                }),
            )]
            .into_iter()
            .collect(),
        };
        assert_eq!(actual, expected);
    }
}
