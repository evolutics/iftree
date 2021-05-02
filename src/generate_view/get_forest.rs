use super::sanitize_name;
use crate::model;
use std::iter;
use std::path;
use std::vec;

pub fn main(paths: vec::Vec<model::Path>) -> model::FileForest {
    let mut forest = model::FileForest::new();

    for path in paths.into_iter() {
        let reverse_path = get_reverse_path(&path.relative);
        if let Some(filename) = reverse_path.first() {
            let file = get_file(filename, path);
            add_file(&mut forest, reverse_path, file);
        }
    }

    let mut index = 0;
    overwrite_indices_in_order(&mut forest, &mut index);

    forest
}

fn get_reverse_path(path: &model::RelativePath) -> vec::Vec<String> {
    path::Path::new(&path.0)
        .iter()
        .rev()
        .map(|name| name.to_string_lossy().to_string())
        .collect()
}

fn get_file(name: &str, path: model::Path) -> model::File {
    let identifier = sanitize_name::main(name, sanitize_name::Convention::ScreamingSnakeCase);
    let identifier = quote::format_ident!("{}", identifier);
    let relative_path = path.relative;
    let absolute_path = path.absolute;
    model::File {
        identifier,
        index: 0,
        relative_path,
        absolute_path,
    }
}

fn add_file(parent: &mut model::FileForest, mut reverse_path: vec::Vec<String>, file: model::File) {
    match reverse_path.pop() {
        None => {}

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_path, file, &name);
                parent.insert(name, child);
            }

            Some(model::FileTree::File(_)) => {}

            Some(model::FileTree::Folder(model::Folder { forest, .. })) => {
                add_file(forest, reverse_path, file)
            }
        },
    }
}

fn get_singleton_tree(
    reverse_path: vec::Vec<String>,
    file: model::File,
    root: &str,
) -> model::FileTree {
    let parents = get_folder_identifiers(
        &reverse_path
            .iter()
            .skip(1)
            .map(|name| name.as_ref())
            .chain(iter::once(root))
            .collect::<vec::Vec<_>>(),
    );

    let mut tree = model::FileTree::File(file);

    for (child, parent) in reverse_path.into_iter().zip(parents.into_iter()) {
        let forest = vec![(child, tree)].into_iter().collect();
        tree = model::FileTree::Folder(model::Folder {
            identifier: parent,
            forest,
        });
    }

    tree
}

fn get_folder_identifiers(names: &[&str]) -> vec::Vec<syn::Ident> {
    names
        .iter()
        .map(|name| {
            let identifier = sanitize_name::main(name, sanitize_name::Convention::SnakeCase);
            quote::format_ident!("{}", identifier)
        })
        .collect()
}

fn overwrite_indices_in_order(forest: &mut model::FileForest, index: &mut usize) {
    for tree in forest.values_mut() {
        match tree {
            model::FileTree::File(file) => {
                file.index = *index;
                *index += 1;
            }

            model::FileTree::Folder(model::Folder { forest, .. }) => {
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

        let expected = model::FileForest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_files() {
        let actual = main(vec![
            model::Path {
                relative: model::RelativePath::from("B"),
                absolute: String::from("/a/B"),
            },
            model::Path {
                relative: model::RelativePath::from("c"),
                absolute: String::from("/a/c"),
            },
        ]);

        let expected = vec![
            (
                String::from('B'),
                model::FileTree::File(model::File {
                    identifier: quote::format_ident!("r#B"),
                    index: 0,
                    relative_path: model::RelativePath::from("B"),
                    absolute_path: String::from("/a/B"),
                }),
            ),
            (
                String::from('c'),
                model::FileTree::File(model::File {
                    identifier: quote::format_ident!("r#C"),
                    index: 1,
                    relative_path: model::RelativePath::from("c"),
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
                relative: model::RelativePath::from("a"),
                absolute: String::from("/a"),
            },
            model::Path {
                relative: model::RelativePath::from("b/a/b"),
                absolute: String::from("/b/a/b"),
            },
            model::Path {
                relative: model::RelativePath::from("b/c"),
                absolute: String::from("/b/c"),
            },
        ]);

        let expected = vec![
            (
                String::from('a'),
                model::FileTree::File(model::File {
                    identifier: quote::format_ident!("r#A"),
                    index: 0,
                    relative_path: model::RelativePath::from("a"),
                    absolute_path: String::from("/a"),
                }),
            ),
            (
                String::from('b'),
                model::FileTree::Folder(model::Folder {
                    identifier: quote::format_ident!("r#b"),
                    forest: vec![
                        (
                            String::from('a'),
                            model::FileTree::Folder(model::Folder {
                                identifier: quote::format_ident!("r#a"),
                                forest: vec![(
                                    String::from('b'),
                                    model::FileTree::File(model::File {
                                        identifier: quote::format_ident!("r#B"),
                                        index: 1,
                                        relative_path: model::RelativePath::from("b/a/b"),
                                        absolute_path: String::from("/b/a/b"),
                                    }),
                                )]
                                .into_iter()
                                .collect(),
                            }),
                        ),
                        (
                            String::from('c'),
                            model::FileTree::File(model::File {
                                identifier: quote::format_ident!("r#C"),
                                index: 2,
                                relative_path: model::RelativePath::from("b/c"),
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
}
