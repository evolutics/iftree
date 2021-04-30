use super::sanitize_name;
use crate::data;
use crate::model;
use std::iter;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    paths: &[model::Path],
) -> model::Result<model::FileForest> {
    Ok(if configuration.identifiers {
        let forest = get_forest(paths)?;
        vec![(
            String::from(data::BASE_MODULE_NAME),
            model::FileTree::Folder(forest),
        )]
        .into_iter()
        .collect()
    } else {
        model::FileForest::new()
    })
}

fn get_forest(paths: &[model::Path]) -> model::Result<model::FileForest> {
    let mut forest = model::FileForest::new();

    for (index, path) in paths.iter().enumerate() {
        let context = Context {
            index,
            path,
            name: String::new(),
        };

        match add_file(&mut forest, context) {
            None => Ok(()),
            Some(Collision { name, competitors }) => Err(model::Error::NameCollision {
                name,
                competitors: competitors
                    .into_iter()
                    .map(|index| paths[index].relative.clone())
                    .collect(),
            }),
        }?
    }

    Ok(forest)
}

struct Context<'a> {
    index: usize,
    path: &'a model::Path,
    name: String,
}

fn add_file(forest: &mut model::FileForest, context: Context) -> Option<Collision> {
    let mut reverse_file_path = get_reverse_file_path(context.path);
    add_file_recursively(forest, &mut reverse_file_path, context)
}

struct Collision {
    name: String,
    competitors: vec::Vec<usize>,
}

fn get_reverse_file_path(path: &model::Path) -> vec::Vec<String> {
    path::Path::new(&path.relative.0)
        .iter()
        .rev()
        .enumerate()
        .map(|(index, name)| {
            let name = name.to_string_lossy();
            let convention = if index == 0 {
                sanitize_name::Convention::ScreamingSnakeCase
            } else {
                sanitize_name::Convention::SnakeCase
            };
            sanitize_name::main(&name, convention)
        })
        .collect()
}

fn add_file_recursively(
    parent: &mut model::FileForest,
    reverse_file_path: &mut vec::Vec<String>,
    context: Context,
) -> Option<Collision> {
    match reverse_file_path.pop() {
        None => {
            let competitors = get_simple_sample_index(parent)
                .into_iter()
                .chain(iter::once(context.index))
                .collect();
            Some(Collision {
                name: context.name,
                competitors,
            })
        }

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_file_path.to_vec(), context.index);
                parent.insert(name, child);
                None
            }

            Some(model::FileTree::File { index }) => Some(Collision {
                name,
                competitors: vec![*index, context.index],
            }),

            Some(model::FileTree::Folder(child)) => {
                add_file_recursively(child, reverse_file_path, Context { name, ..context })
            }
        },
    }
}

fn get_simple_sample_index(forest: &model::FileForest) -> Option<usize> {
    for tree in forest.values() {
        match tree {
            model::FileTree::File { index } => return Some(*index),
            model::FileTree::Folder(_) => (),
        }
    }
    for tree in forest.values() {
        match tree {
            model::FileTree::File { .. } => (),
            model::FileTree::Folder(forest) => return get_simple_sample_index(forest),
        }
    }
    None
}

fn get_singleton_tree(reverse_file_path: vec::Vec<String>, index: usize) -> model::FileTree {
    let mut child = model::FileTree::File { index };

    for name in reverse_file_path.into_iter() {
        child = model::FileTree::Folder(vec![(name, child)].into_iter().collect())
    }

    child
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

        let actual = actual.unwrap();
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

        let actual = actual.unwrap();
        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(model::FileForest::new()),
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
                    relative: model::RelativePath::from("a"),
                    ..model::stubs::path()
                },
                model::Path {
                    relative: model::RelativePath::from("b"),
                    ..model::stubs::path()
                },
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(
                vec![
                    (String::from("r#A"), model::FileTree::File { index: 0 }),
                    (String::from("r#B"), model::FileTree::File { index: 1 }),
                ]
                .into_iter()
                .collect(),
            ),
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

        let actual = actual.unwrap();
        let expected = vec![(
            String::from("base"),
            model::FileTree::Folder(
                vec![
                    (String::from("r#A"), model::FileTree::File { index: 0 }),
                    (
                        String::from("r#b"),
                        model::FileTree::Folder(
                            vec![
                                (
                                    String::from("r#a"),
                                    model::FileTree::Folder(
                                        vec![(
                                            String::from("r#B"),
                                            model::FileTree::File { index: 1 },
                                        )]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                (String::from("r#C"), model::FileTree::File { index: 2 }),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[cfg(test)]
    mod name_collision {
        use super::*;

        #[test]
        fn given_file_collides_with_file_it_errs() {
            let actual = main(
                &model::Configuration {
                    identifiers: true,
                    ..model::stubs::configuration()
                },
                &[
                    model::Path {
                        relative: model::RelativePath::from("a/B"),
                        ..model::stubs::path()
                    },
                    model::Path {
                        relative: model::RelativePath::from("a/b"),
                        ..model::stubs::path()
                    },
                ],
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::NameCollision {
                name: String::from("r#B"),
                competitors: vec![
                    model::RelativePath::from("a/B"),
                    model::RelativePath::from("a/b"),
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_file_collides_with_folder_it_errs() {
            let actual = main(
                &model::Configuration {
                    identifiers: true,
                    ..model::stubs::configuration()
                },
                &[
                    model::Path {
                        relative: model::RelativePath::from("a/-/b"),
                        ..model::stubs::path()
                    },
                    model::Path {
                        relative: model::RelativePath::from("a/~"),
                        ..model::stubs::path()
                    },
                ],
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::NameCollision {
                name: String::from("r#__"),
                competitors: vec![
                    model::RelativePath::from("a/-/b"),
                    model::RelativePath::from("a/~"),
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_folder_collides_with_file_it_errs() {
            let actual = main(
                &model::Configuration {
                    identifiers: true,
                    ..model::stubs::configuration()
                },
                &[
                    model::Path {
                        relative: model::RelativePath::from("a/-"),
                        ..model::stubs::path()
                    },
                    model::Path {
                        relative: model::RelativePath::from("a/~/b"),
                        ..model::stubs::path()
                    },
                ],
            );

            let actual = actual.unwrap_err();
            let expected = model::Error::NameCollision {
                name: String::from("r#__"),
                competitors: vec![
                    model::RelativePath::from("a/-"),
                    model::RelativePath::from("a/~/b"),
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn given_folder_collides_with_folder_it_merges() {
            let actual = main(
                &model::Configuration {
                    identifiers: true,
                    ..model::stubs::configuration()
                },
                &[
                    model::Path {
                        relative: model::RelativePath::from("A/b"),
                        ..model::stubs::path()
                    },
                    model::Path {
                        relative: model::RelativePath::from("a/c"),
                        ..model::stubs::path()
                    },
                ],
            );

            let actual = actual.unwrap();
            let expected = vec![(
                String::from("base"),
                model::FileTree::Folder(
                    vec![(
                        String::from("r#a"),
                        model::FileTree::Folder(
                            vec![
                                (String::from("r#B"), model::FileTree::File { index: 0 }),
                                (String::from("r#C"), model::FileTree::File { index: 1 }),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    )]
                    .into_iter()
                    .collect(),
                ),
            )]
            .into_iter()
            .collect();
            assert_eq!(actual, expected);
        }
    }
}
