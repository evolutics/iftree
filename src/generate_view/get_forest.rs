use super::sanitize_name;
use crate::model;
use std::iter;
use std::path;
use std::vec;

pub fn main(paths: vec::Vec<model::Path>) -> model::Result<model::Forest> {
    let mut forest = model::Forest::new();

    for path in paths.into_iter() {
        let reverse_path = get_reverse_path(&path.relative);
        if let Some(filename) = reverse_path.first() {
            let file = get_file(filename, path);
            add_file(&mut forest, reverse_path, file)?;
        }
    }

    let mut index = 0;
    overwrite_indices_in_order(&mut forest, &mut index);

    Ok(forest)
}

fn get_reverse_path(path: &str) -> vec::Vec<String> {
    path::Path::new(path)
        .iter()
        .rev()
        .map(|name| name.to_string_lossy().to_string())
        .collect()
}

fn get_file(name: &str, path: model::Path) -> model::File {
    let identifier = sanitize_name::main(name, sanitize_name::Convention::ScreamingSnakeCase);
    let relative_path = path.relative;
    let absolute_path = path.absolute;
    model::File {
        identifier,
        index: 0,
        relative_path,
        absolute_path,
    }
}

fn add_file(
    parent: &mut model::Forest,
    mut reverse_path: vec::Vec<String>,
    file: model::File,
) -> model::Result<()> {
    match reverse_path.pop() {
        None => Err(model::Error::UnexpectedPathCollision(path::PathBuf::from(
            file.relative_path,
        ))),

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_path, file, &name);
                parent.insert(name, child);
                Ok(())
            }

            Some(model::Tree::File(_)) => Err(model::Error::UnexpectedPathCollision(
                path::PathBuf::from(file.relative_path),
            )),

            Some(model::Tree::Folder(model::Folder { forest, .. })) => {
                add_file(forest, reverse_path, file)
            }
        },
    }
}

fn get_singleton_tree(
    reverse_path: vec::Vec<String>,
    file: model::File,
    root: &str,
) -> model::Tree {
    let parents = get_folder_identifiers(
        &reverse_path
            .iter()
            .skip(1)
            .map(|name| name.as_ref())
            .chain(iter::once(root))
            .collect::<vec::Vec<_>>(),
    );

    let mut tree = model::Tree::File(file);

    for (child, parent) in reverse_path.into_iter().zip(parents.into_iter()) {
        let forest = vec![(child, tree)].into_iter().collect();
        tree = model::Tree::Folder(model::Folder {
            identifier: parent,
            forest,
        });
    }

    tree
}

fn get_folder_identifiers(names: &[&str]) -> vec::Vec<syn::Ident> {
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
                relative: String::from('B'),
                absolute: String::from("/a/B"),
            },
            model::Path {
                relative: String::from('c'),
                absolute: String::from("/a/c"),
            },
        ]);

        let actual = actual.unwrap();
        let expected = vec![
            (
                String::from('B'),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#B"),
                    index: 0,
                    relative_path: String::from('B'),
                    absolute_path: String::from("/a/B"),
                }),
            ),
            (
                String::from('c'),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#C"),
                    index: 1,
                    relative_path: String::from('c'),
                    absolute_path: String::from("/a/c"),
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
                relative: String::from('a'),
                absolute: String::from("/a"),
            },
            model::Path {
                relative: String::from("b/a/b"),
                absolute: String::from("/b/a/b"),
            },
            model::Path {
                relative: String::from("b/c"),
                absolute: String::from("/b/c"),
            },
        ]);

        let actual = actual.unwrap();
        let expected = vec![
            (
                String::from('a'),
                model::Tree::File(model::File {
                    identifier: quote::format_ident!("r#A"),
                    index: 0,
                    relative_path: String::from('a'),
                    absolute_path: String::from("/a"),
                }),
            ),
            (
                String::from('b'),
                model::Tree::Folder(model::Folder {
                    identifier: quote::format_ident!("r#b"),
                    forest: vec![
                        (
                            String::from('a'),
                            model::Tree::Folder(model::Folder {
                                identifier: quote::format_ident!("r#a"),
                                forest: vec![(
                                    String::from('b'),
                                    model::Tree::File(model::File {
                                        identifier: quote::format_ident!("r#B"),
                                        index: 1,
                                        relative_path: String::from("b/a/b"),
                                        absolute_path: String::from("/b/a/b"),
                                    }),
                                )]
                                .into_iter()
                                .collect(),
                            }),
                        ),
                        (
                            String::from('c'),
                            model::Tree::File(model::File {
                                identifier: quote::format_ident!("r#C"),
                                index: 2,
                                relative_path: String::from("b/c"),
                                absolute_path: String::from("/b/c"),
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
    fn given_path_collision_it_errs() {
        let actual = main(vec![
            model::Path {
                relative: String::from("a/b"),
                ..model::stubs::path()
            },
            model::Path {
                relative: String::from("a/b"),
                ..model::stubs::path()
            },
        ]);

        let actual = actual.unwrap_err();
        let expected = model::Error::UnexpectedPathCollision(path::PathBuf::from("a/b"));
        assert_eq!(actual, expected);
    }
}
