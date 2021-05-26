use super::sanitize_name;
use crate::model;
use std::array;
use std::iter;
use std::path;

pub fn main(paths: Vec<model::Path>) -> model::Result<model::Forest> {
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

fn get_reverse_path(components: &[String]) -> Vec<String> {
    components.iter().rev().map(String::from).collect()
}

fn get_file(name: &str, path: model::Path) -> model::File {
    let identifier = sanitize_name::main(name, sanitize_name::Convention::ScreamingSnakeCase);
    let relative_path = path.relative.join("/");
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
    mut reverse_path: Vec<String>,
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
        let forest = array::IntoIter::new([(child, tree)]).collect();
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
                relative: vec![String::from('B')],
                absolute: String::from("/a/B"),
            },
            model::Path {
                relative: vec![String::from('c')],
                absolute: String::from("/a/c"),
            },
        ]);

        let actual = actual.unwrap();
        let expected = array::IntoIter::new([
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
        ])
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_folders() {
        let actual = main(vec![
            model::Path {
                relative: vec![String::from('a')],
                absolute: String::from("/a"),
            },
            model::Path {
                relative: vec![String::from('b'), String::from('a'), String::from('b')],
                absolute: String::from("/b/a/b"),
            },
            model::Path {
                relative: vec![String::from('b'), String::from('c')],
                absolute: String::from("/b/c"),
            },
        ]);

        let actual = actual.unwrap();
        let expected = array::IntoIter::new([
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
                    forest: array::IntoIter::new([
                        (
                            String::from('a'),
                            model::Tree::Folder(model::Folder {
                                identifier: quote::format_ident!("r#a"),
                                forest: array::IntoIter::new([(
                                    String::from('b'),
                                    model::Tree::File(model::File {
                                        identifier: quote::format_ident!("r#B"),
                                        index: 1,
                                        relative_path: String::from("b/a/b"),
                                        absolute_path: String::from("/b/a/b"),
                                    }),
                                )])
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
                    ])
                    .collect(),
                }),
            ),
        ])
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_path_collision_it_errs() {
        let actual = main(vec![
            model::Path {
                relative: vec![String::from('a'), String::from('b')],
                ..model::stubs::path()
            },
            model::Path {
                relative: vec![String::from('a'), String::from('b')],
                ..model::stubs::path()
            },
        ]);

        let actual = actual.unwrap_err();
        let expected = model::Error::UnexpectedPathCollision(path::PathBuf::from("a/b"));
        assert_eq!(actual, expected);
    }
}
