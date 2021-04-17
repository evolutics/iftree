use super::sanitize_to_identifier;
use crate::data;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    files: &[model::File],
) -> model::Result<model::FileForest> {
    Ok(if configuration.module_tree {
        let forest = get_forest(files)?;
        vec![(
            String::from(data::BASE_MODULE_IDENTIFIER),
            model::FileTree::Folder(forest),
        )]
        .into_iter()
        .collect()
    } else {
        model::FileForest::new()
    })
}

fn get_forest(files: &[model::File]) -> model::Result<model::FileForest> {
    let mut forest = model::FileForest::new();

    for (index, file) in files.iter().enumerate() {
        match add_file(&mut forest, Context { index, file }) {
            None => Ok(()),
            Some(Collision {
                identifier,
                competitors,
            }) => Err(model::Error::NameCollision {
                identifier,
                competitors: competitors
                    .into_iter()
                    .map(|index| files[index].relative_path.clone())
                    .collect(),
            }),
        }?
    }

    Ok(forest)
}

fn add_file(forest: &mut model::FileForest, context: Context) -> Option<Collision> {
    let mut reverse_file_path = get_reverse_file_path(&context.file);
    add_file_recursively(forest, &mut reverse_file_path, context)
}

struct Context<'a> {
    index: usize,
    file: &'a model::File,
}

struct Collision {
    identifier: String,
    competitors: vec::Vec<usize>,
}

fn get_reverse_file_path(file: &model::File) -> vec::Vec<String> {
    path::Path::new(&file.relative_path.0)
        .iter()
        .rev()
        .enumerate()
        .map(|(index, name)| {
            let name = name.to_string_lossy();
            let convention = if index == 0 {
                sanitize_to_identifier::Convention::ScreamingSnakeCase
            } else {
                sanitize_to_identifier::Convention::SnakeCase
            };
            sanitize_to_identifier::main(&name, convention)
        })
        .collect()
}

fn add_file_recursively(
    parent: &mut model::FileForest,
    reverse_file_path: &mut vec::Vec<String>,
    context: Context,
) -> Option<Collision> {
    match reverse_file_path.pop() {
        None => None,

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_file_path.to_vec(), context.index);
                parent.insert(name, child);
                None
            }

            Some(model::FileTree::File { index }) => Some(Collision {
                identifier: name,
                competitors: vec![*index, context.index],
            }),

            Some(model::FileTree::Folder(child)) => {
                add_file_recursively(child, reverse_file_path, context)
            }
        },
    }
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
    fn handles_no_module_tree() {
        let actual = main(
            &model::Configuration {
                module_tree: false,
                ..model::stubs::configuration()
            },
            &[model::stubs::file()],
        );

        let actual = actual.unwrap();
        let expected = model::FileForest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn handles_empty_set() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
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
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[
                model::File {
                    relative_path: model::RelativePath::from("a"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("b"),
                    ..model::stubs::file()
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
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[
                model::File {
                    relative_path: model::RelativePath::from("a"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("b/a/b"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("b/c"),
                    ..model::stubs::file()
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

    #[test]
    fn given_name_collision_it_errs() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[
                model::File {
                    relative_path: model::RelativePath::from("A"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("a"),
                    ..model::stubs::file()
                },
            ],
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::NameCollision {
            identifier: String::from("r#A"),
            competitors: vec![
                model::RelativePath::from("A"),
                model::RelativePath::from("a"),
            ],
        };
        assert_eq!(actual, expected);
    }
}
