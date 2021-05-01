use super::sanitize_name;
use crate::data;
use crate::model;
use std::iter;
use std::path;
use std::vec;

pub fn main(configuration: &model::Configuration, paths: &[model::Path]) -> model::FileForest {
    if configuration.identifiers {
        let identifier = quote::format_ident!("{}", data::BASE_MODULE_NAME);
        let forest = get_forest(paths);
        vec![(
            String::from(data::BASE_MODULE_NAME),
            model::FileTree::Folder(model::Folder { identifier, forest }),
        )]
        .into_iter()
        .collect()
    } else {
        model::FileForest::new()
    }
}

fn get_forest(paths: &[model::Path]) -> model::FileForest {
    let mut forest = model::FileForest::new();

    for (index, path) in paths.iter().enumerate() {
        let reverse_path = get_reverse_path(&path.relative);
        if let Some(filename) = reverse_path.first() {
            let file = get_file(filename, index);
            add_file(&mut forest, reverse_path, file);
        }
    }

    forest
}

fn get_reverse_path(path: &model::RelativePath) -> vec::Vec<String> {
    path::Path::new(&path.0)
        .iter()
        .rev()
        .map(|name| name.to_string_lossy().to_string())
        .collect()
}

fn get_file(name: &str, index: usize) -> model::File {
    let identifier = sanitize_name::main(name, sanitize_name::Convention::ScreamingSnakeCase);
    let identifier = quote::format_ident!("{}", identifier);
    model::File { identifier, index }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_no_identifiers() {
        let actual = main(
            &model::Configuration {
                identifiers: false,
                ..model::stubs::configuration()
            },
            &[model::stubs::path()],
        );

        let expected = model::FileForest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_empty_set() {
        let actual = main(
            &model::Configuration {
                identifiers: true,
                ..model::stubs::configuration()
            },
            &[],
        );

        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(model::Folder {
                identifier: quote::format_ident!("base"),
                forest: model::FileForest::new(),
            }),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_files() {
        let actual = main(
            &model::Configuration {
                identifiers: true,
                ..model::stubs::configuration()
            },
            &[
                model::Path {
                    relative: model::RelativePath::from("A"),
                    ..model::stubs::path()
                },
                model::Path {
                    relative: model::RelativePath::from("b"),
                    ..model::stubs::path()
                },
            ],
        );

        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(model::Folder {
                identifier: quote::format_ident!("base"),
                forest: vec![
                    (
                        String::from('A'),
                        model::FileTree::File(model::File {
                            identifier: quote::format_ident!("r#A"),
                            index: 0,
                        }),
                    ),
                    (
                        String::from('b'),
                        model::FileTree::File(model::File {
                            identifier: quote::format_ident!("r#B"),
                            index: 1,
                        }),
                    ),
                ]
                .into_iter()
                .collect(),
            }),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_folders() {
        let actual = main(
            &model::Configuration {
                identifiers: true,
                ..model::stubs::configuration()
            },
            &[
                model::Path {
                    relative: model::RelativePath::from("a"),
                    ..model::stubs::path()
                },
                model::Path {
                    relative: model::RelativePath::from("b/a/b"),
                    ..model::stubs::path()
                },
                model::Path {
                    relative: model::RelativePath::from("b/c"),
                    ..model::stubs::path()
                },
            ],
        );

        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(model::Folder {
                identifier: quote::format_ident!("base"),
                forest: vec![
                    (
                        String::from('a'),
                        model::FileTree::File(model::File {
                            identifier: quote::format_ident!("r#A"),
                            index: 0,
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
                                    }),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        }),
                    ),
                ]
                .into_iter()
                .collect(),
            }),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }
}
