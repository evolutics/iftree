use super::get_array;
use super::get_forest;
use super::get_initializer;
use crate::model;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    type_: model::Type<()>,
    paths: vec::Vec<model::Path>,
) -> model::Result<model::View> {
    let initializer = get_initializer::main(configuration, type_.structure)?;
    let array = get_array::main(paths);
    let forest = get_forest::main(configuration, &array)?;
    Ok(model::View {
        type_: type_.name,
        initializer,
        array,
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
            initializer: model::Initializer::Macro(String::from("abc")),
            array: vec![model::Path {
                relative: model::RelativePath::from("b"),
                absolute: String::from("/a/b"),
            }],
            forest: vec![(
                String::from("base"),
                model::FileTree::Folder(
                    vec![(
                        String::from("r#B"),
                        model::FileTree::File(model::File { index: 0 }),
                    )]
                    .into_iter()
                    .collect(),
                ),
            )]
            .into_iter()
            .collect(),
        };
        assert_eq!(actual, expected);
    }
}
