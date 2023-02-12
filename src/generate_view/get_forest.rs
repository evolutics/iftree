use super::sanitize_name;
use crate::model;
use std::iter;

pub fn main(paths: Vec<model::Path>) -> model::Result<model::Forest> {
    let mut forest = model::Forest::new();

    for path in paths.into_iter() {
        add_path(&mut forest, path)?;
    }

    let mut index = 0;
    overwrite_indices_in_order(&mut forest, &mut index);

    Ok(forest)
}

fn add_path(forest: &mut model::Forest, path: model::Path) -> model::Result<()> {
    match path.relative.last() {
        None => Err(model::Error::UnexpectedEmptyRelativePath {
            absolute_path: path.absolute.into(),
        }),

        Some(filename) => {
            let file = model::File {
                identifier: sanitize_name::main(
                    filename,
                    sanitize_name::Convention::ScreamingSnakeCase,
                ),
                index: 0,
                relative_path: path.relative.join(NORMALIZED_FOLDER_SEPARATOR),
                absolute_path: path.absolute,
            };

            let mut reverse_path = path.relative;
            reverse_path.reverse();

            add_file(forest, reverse_path, file)
        }
    }
}

const NORMALIZED_FOLDER_SEPARATOR: &str = "/";

fn add_file(
    parent: &mut model::Forest,
    mut reverse_path: Vec<String>,
    file: model::File,
) -> model::Result<()> {
    match reverse_path.pop() {
        None => Err(model::Error::UnexpectedPathCollision(
            file.relative_path.into(),
        )),

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_path, file, &name);
                parent.insert(name, child);
                Ok(())
            }

            Some(model::Tree::File(_)) => Err(model::Error::UnexpectedPathCollision(
                file.relative_path.into(),
            )),

            Some(model::Tree::Folder(model::Folder { forest, .. })) => {
                add_file(forest, reverse_path, file)
            }
        },
    }
}

fn get_singleton_tree(reverse_path: Vec<String>, file: model::File, root: &str) -> model::Tree {
    let parents = get_folder_identifiers(
        &reverse_path
            .iter()
            .skip(1)
            .map(|name| name.as_ref())
            .chain(iter::once(root))
            .collect::<Vec<_>>(),
    );

    let mut tree = model::Tree::File(file);

    for (child, parent) in reverse_path.into_iter().zip(parents.into_iter()) {
        let forest = [(child, tree)].into_iter().collect();
        tree = model::Tree::Folder(model::Folder {
            identifier: parent,
            forest,
        });
    }

    tree
}

fn get_folder_identifiers(names: &[&str]) -> Vec<syn::Ident> {
    names
        .iter()
        .map(|name| sanitize_name::main(name, sanitize_name::Convention::SnakeCase))
        .collect()
}

fn overwrite_indices_in_order(forest: &mut model::Forest, index: &mut usize) {
    for tree in forest.values_mut() {
        match tree {
            model::Tree::File(file) => {
                file.index = *index;
                *index += 1;
            }

            model::Tree::Folder(model::Folder { forest, .. }) => {
                overwrite_indices_in_order(forest, index)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty_set() {
        let actual = main(vec![]);

        let actual = actual.unwrap();
        let expected = model::Forest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_files() {
        let actual = main(vec![
            model::Path {
                relative: vec!['B'.into()],
                absolute: "/a/B".into(),
            },
            model::Path {
                relative: vec!['c'.into()],
                absolute: "/a/c".into(),
            },
        ]);

        let actual = actual.unwrap();
        let expected = [
            (
                'B'.into(),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#B"),
                    index: 0,
                    relative_path: 'B'.into(),
                    absolute_path: "/a/B".into(),
                }),
            ),
            (
                'c'.into(),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#C"),
                    index: 1,
                    relative_path: 'c'.into(),
                    absolute_path: "/a/c".into(),
                }),
            ),
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_folders() {
        let actual = main(vec![
            model::Path {
                relative: vec!['a'.into()],
                absolute: "/a".into(),
            },
            model::Path {
                relative: vec!['b'.into(), 'a'.into(), 'b'.into()],
                absolute: "/b/a/b".into(),
            },
            model::Path {
                relative: vec!['b'.into(), 'c'.into()],
                absolute: "/b/c".into(),
            },
        ]);

        let actual = actual.unwrap();
        let expected = [
            (
                'a'.into(),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#A"),
                    index: 0,
                    relative_path: 'a'.into(),
                    absolute_path: "/a".into(),
                }),
            ),
            (
                'b'.into(),
                model::Tree::Folder(model::Folder {
                    identifier: quote::format_ident!("r#b"),
                    forest: [
                        (
                            'a'.into(),
                            model::Tree::Folder(model::Folder {
                                identifier: quote::format_ident!("r#a"),
                                forest: [(
                                    'b'.into(),
                                    model::Tree::File(model::File {
                                        identifier: quote::format_ident!("r#B"),
                                        index: 1,
                                        relative_path: "b/a/b".into(),
                                        absolute_path: "/b/a/b".into(),
                                    }),
                                )]
                                .into_iter()
                                .collect(),
                            }),
                        ),
                        (
                            'c'.into(),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("r#C"),
                                index: 2,
                                relative_path: "b/c".into(),
                                absolute_path: "/b/c".into(),
                            }),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_empty_relative_path_it_errs() {
        let actual = main(vec![model::Path {
            relative: vec![],
            absolute: "/a/b".into(),
        }]);

        let actual = actual.unwrap_err();
        let expected = model::Error::UnexpectedEmptyRelativePath {
            absolute_path: "/a/b".into(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_path_collision_it_errs() {
        let actual = main(vec![
            model::Path {
                relative: vec!['a'.into(), 'b'.into()],
                ..model::stubs::path()
            },
            model::Path {
                relative: vec!['a'.into(), 'b'.into()],
                ..model::stubs::path()
            },
        ]);

        let actual = actual.unwrap_err();
        let expected = model::Error::UnexpectedPathCollision("a/b".into());
        assert_eq!(actual, expected);
    }
}
