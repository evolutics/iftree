use super::sanitize_to_identifier;
use crate::model;
use std::path;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    files: &[model::File],
) -> model::Result<model::FileForest> {
    let mut forest = model::FileForest::new();
    if configuration.module_tree {
        for (index, file) in files.iter().enumerate() {
            let context = Context { index, file };
            add_file(&mut forest, context)?;
        }
    }
    Ok(forest)
}

struct Context<'a> {
    index: usize,
    file: &'a model::File,
}

fn add_file(forest: &mut model::FileForest, context: Context) -> model::Result<()> {
    let mut reverse_file_path = get_reverse_file_path(&context.file);
    add_file_recursively(forest, &mut reverse_file_path, context)
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
) -> model::Result<()> {
    match reverse_file_path.pop() {
        None => Ok(()),

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_file_path.to_vec(), context.index);
                parent.insert(name, child);
                Ok(())
            }

            Some(model::FileTree::File(_)) => {
                Err(model::Error::NameCollision(model::NameCollisionError {
                    collider: context.file.relative_path.clone(),
                    identifier: name,
                }))
            }

            Some(model::FileTree::Folder(child)) => {
                add_file_recursively(child, reverse_file_path, context)
            }
        },
    }
}

fn get_singleton_tree(reverse_file_path: vec::Vec<String>, index: usize) -> model::FileTree {
    let mut child = model::FileTree::File(index);

    for name in reverse_file_path.into_iter() {
        child = model::FileTree::Folder(vec![(name, child)].into_iter().collect())
    }

    child
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_module_tree_is_not_configured_it_gets_empty() {
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
    fn gets_empty_set() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[],
        );

        let actual = actual.unwrap();
        let expected = model::FileForest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_files() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[
                model::File {
                    relative_path: model::RelativePath::from("menu.json"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("translations.csv"),
                    ..model::stubs::file()
                },
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            (String::from("r#MENU_JSON"), model::FileTree::File(0)),
            (String::from("r#TRANSLATIONS_CSV"), model::FileTree::File(1)),
        ]
        .into_iter()
        .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_folders() {
        let actual = main(
            &model::Configuration {
                module_tree: true,
                ..model::stubs::configuration()
            },
            &[
                model::File {
                    relative_path: model::RelativePath::from("credits.md"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("world/levels/tutorial.json"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("world/physical_constants.json"),
                    ..model::stubs::file()
                },
            ],
        );

        let actual = actual.unwrap();
        let expected = vec![
            (String::from("r#CREDITS_MD"), model::FileTree::File(0)),
            (
                String::from("r#world"),
                model::FileTree::Folder(
                    vec![
                        (
                            String::from("r#levels"),
                            model::FileTree::Folder(
                                vec![(String::from("r#TUTORIAL_JSON"), model::FileTree::File(1))]
                                    .into_iter()
                                    .collect(),
                            ),
                        ),
                        (
                            String::from("r#PHYSICAL_CONSTANTS_JSON"),
                            model::FileTree::File(2),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ),
        ]
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
                    relative_path: model::RelativePath::from("CREDITS.md"),
                    ..model::stubs::file()
                },
                model::File {
                    relative_path: model::RelativePath::from("credits.md"),
                    ..model::stubs::file()
                },
            ],
        );

        let actual = actual.unwrap_err();
        let expected = model::Error::NameCollision(model::NameCollisionError {
            collider: model::RelativePath::from("credits.md"),
            identifier: String::from("r#CREDITS_MD"),
        });
        assert_eq!(actual, expected);
    }
}
