use super::get_array;
use super::get_forest;
use super::get_templates;
use crate::model;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    type_: model::Type<()>,
    files: vec::Vec<model::File>,
) -> model::Result<model::View> {
    let type_ = get_templates::main(configuration, type_)?;
    let array = get_array::main(files);
    let forest = get_forest::main(configuration, &array)?;
    Ok(model::View {
        type_,
        array,
        forest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn handles() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
                field_templates: vec![(
                    model::FieldIdentifier::Anonymous,
                    model::Template::Content,
                )]
                .into_iter()
                .collect(),
                ..model::stubs::configuration()
            },
            model::Type {
                identifier: quote::format_ident!("Resource"),
                structure: model::TypeStructure::TypeAlias(()),
            },
            vec![model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: path::PathBuf::from("/a/b"),
            }],
        );

        let actual = actual.unwrap();
        let expected = model::View {
            type_: model::Type {
                identifier: quote::format_ident!("Resource"),
                structure: model::TypeStructure::TypeAlias(model::Template::Content),
            },
            array: vec![model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: path::PathBuf::from("/a/b"),
            }],
            forest: vec![(
                String::from("base"),
                model::FileTree::Folder(
                    vec![(String::from("r#B"), model::FileTree::File { index: 0 })]
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
