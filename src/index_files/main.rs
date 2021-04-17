use super::get_files;
use super::get_forest;
use super::get_templates;
use crate::model;

pub fn main(
    configuration: &model::Configuration,
    type_: model::Type<()>,
    system_data: model::SystemData,
) -> model::Result<model::FileIndex> {
    let type_ = get_templates::main(configuration, type_)?;
    let files = get_files::main(&system_data.base_folder, system_data.paths)?;
    let forest = get_forest::main(configuration, &files)?;
    Ok(model::FileIndex {
        type_,
        array: files,
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
            model::SystemData {
                base_folder: path::PathBuf::from("/a"),
                paths: vec![path::PathBuf::from("/a/b")],
            },
        );

        let actual = actual.unwrap();
        let expected = model::FileIndex {
            type_: model::Type {
                identifier: quote::format_ident!("Resource"),
                structure: model::TypeStructure::TypeAlias(model::Template::Content),
            },
            array: vec![model::File {
                relative_path: model::RelativePath::from("b"),
                absolute_path: path::PathBuf::from("/a/b"),
            }],
            forest: Some(
                vec![(String::from("r#B"), model::FileTree::File { index: 0 })]
                    .into_iter()
                    .collect(),
            ),
        };
        assert_eq!(actual, expected);
    }
}
