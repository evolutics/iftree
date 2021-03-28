use super::generate_identifier;
use super::sanitize_to_identifier;
use crate::model;
use std::vec;

pub fn main(
    configuration: &model::Configuration,
    mut files: vec::Vec<model::File>,
) -> model::Result<model::FileForest> {
    files.sort_unstable();

    let mut forest = model::FileForest::new();

    let collisions: vec::Vec<model::NameCollision> = files
        .into_iter()
        .filter_map(|file| add_file(&mut forest, file, false))
        .collect();

    if collisions.is_empty() || configuration.resolve_name_collisions {
        for collision in collisions {
            add_file(&mut forest, collision.colliding_file, true);
        }
        Ok(forest)
    } else {
        Err(model::Error::NameCollisions(collisions))
    }
}

fn add_file(
    forest: &mut model::FileForest,
    file: model::File,
    is_to_force: bool,
) -> Option<model::NameCollision> {
    let mut reverse_file_path = get_reverse_file_path(&file);
    add_file_recursively(forest, &mut reverse_file_path, file, is_to_force)
}

fn get_reverse_file_path(file: &model::File) -> vec::Vec<String> {
    file.relative_path
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
    file: model::File,
    is_to_force: bool,
) -> Option<model::NameCollision> {
    match reverse_file_path.pop() {
        None => None,

        Some(name) => match parent.get_mut(&name) {
            None => {
                let child = get_singleton_tree(reverse_file_path.to_vec(), file);
                parent.insert(name, child);
                None
            }

            Some(model::FileTree::File(existing_file)) => {
                if is_to_force {
                    let name = generate_identifier::main(&name, &|name| !parent.contains_key(name));
                    let child = get_singleton_tree(reverse_file_path.to_vec(), file);
                    parent.insert(name, child);
                    None
                } else {
                    Some(model::NameCollision {
                        colliding_file: file,
                        existing_filename: existing_file
                            .relative_path
                            .file_name()
                            .map(|filename| String::from(filename.to_string_lossy())),
                        identifier: name,
                    })
                }
            }

            Some(model::FileTree::Folder(child)) => {
                add_file_recursively(child, reverse_file_path, file, is_to_force)
            }
        },
    }
}

fn get_singleton_tree(reverse_file_path: vec::Vec<String>, file: model::File) -> model::FileTree {
    let mut child = model::FileTree::File(file);

    for name in reverse_file_path.into_iter() {
        let mut parent = model::FileForest::new();
        parent.insert(name, child);
        child = model::FileTree::Folder(parent);
    }

    child
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path;

    #[test]
    fn gets_empty_set() {
        let actual = main(&model::stubs::configuration(), vec![]);

        let actual = actual.unwrap();
        let expected = model::FileForest::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_files() {
        let menu_json = model::File {
            relative_path: path::PathBuf::from("menu.json"),
            ..model::stubs::file()
        };
        let translations_csv = model::File {
            relative_path: path::PathBuf::from("translations.csv"),
            ..model::stubs::file()
        };
        let files = vec![menu_json.clone(), translations_csv.clone()];

        let actual = main(&model::stubs::configuration(), files);

        let actual = actual.unwrap();
        let mut expected = model::FileForest::new();
        expected.insert(
            String::from("r#MENU_JSON"),
            model::FileTree::File(menu_json),
        );
        expected.insert(
            String::from("r#TRANSLATIONS_CSV"),
            model::FileTree::File(translations_csv),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn gets_folders() {
        let credits_md = model::File {
            relative_path: path::PathBuf::from("credits.md"),
            ..model::stubs::file()
        };
        let tutorial_json = model::File {
            relative_path: path::PathBuf::from("world/levels/tutorial.json"),
            ..model::stubs::file()
        };
        let physical_constants_json = model::File {
            relative_path: path::PathBuf::from("world/physical_constants.json"),
            ..model::stubs::file()
        };
        let files = vec![
            credits_md.clone(),
            tutorial_json.clone(),
            physical_constants_json.clone(),
        ];

        let actual = main(&model::stubs::configuration(), files);

        let actual = actual.unwrap();
        let mut levels = model::FileForest::new();
        levels.insert(
            String::from("r#TUTORIAL_JSON"),
            model::FileTree::File(tutorial_json),
        );
        let mut world = model::FileForest::new();
        world.insert(String::from("r#levels"), model::FileTree::Folder(levels));
        world.insert(
            String::from("r#PHYSICAL_CONSTANTS_JSON"),
            model::FileTree::File(physical_constants_json),
        );
        let mut expected = model::FileForest::new();
        expected.insert(
            String::from("r#CREDITS_MD"),
            model::FileTree::File(credits_md),
        );
        expected.insert(String::from("r#world"), model::FileTree::Folder(world));
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_resolve_name_collisions_is_disabled_it_reports_collisions() {
        let credits_md_0 = model::File {
            relative_path: path::PathBuf::from("CREDITS.md"),
            ..model::stubs::file()
        };
        let credits_md_1 = model::File {
            relative_path: path::PathBuf::from("credits.md"),
            ..model::stubs::file()
        };
        let files = vec![credits_md_0, credits_md_1.clone()];

        let actual = main(
            &model::Configuration {
                resolve_name_collisions: false,
                ..model::stubs::configuration()
            },
            files,
        );

        let actual = match actual.unwrap_err() {
            model::Error::NameCollisions(actual) => actual,
            _ => unreachable!(),
        };
        let expected = vec![model::NameCollision {
            colliding_file: credits_md_1,
            existing_filename: Some(String::from("CREDITS.md")),
            identifier: String::from("r#CREDITS_MD"),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn given_resolve_name_collisions_is_enabled_it_resolves_collisions_in_order() {
        let credits_md_0 = model::File {
            relative_path: path::PathBuf::from("credits.md"),
            ..model::stubs::file()
        };
        let credits_md_1 = model::File {
            relative_path: path::PathBuf::from("CREDITS.md"),
            ..model::stubs::file()
        };
        let credits_md_2 = model::File {
            relative_path: path::PathBuf::from("Credits.md"),
            ..model::stubs::file()
        };
        let credits_md_3 = model::File {
            relative_path: path::PathBuf::from("credits.md0"),
            ..model::stubs::file()
        };
        let files = vec![
            credits_md_0.clone(),
            credits_md_1.clone(),
            credits_md_2.clone(),
            credits_md_3.clone(),
        ];

        let actual = main(
            &model::Configuration {
                resolve_name_collisions: true,
                ..model::stubs::configuration()
            },
            files,
        );

        let actual = actual.unwrap();
        let mut expected = model::FileForest::new();
        expected.insert(
            String::from("r#CREDITS_MD"),
            model::FileTree::File(credits_md_1),
        );
        expected.insert(
            String::from("r#CREDITS_MD0"),
            model::FileTree::File(credits_md_3),
        );
        expected.insert(
            String::from("r#CREDITS_MD1"),
            model::FileTree::File(credits_md_2),
        );
        expected.insert(
            String::from("r#CREDITS_MD2"),
            model::FileTree::File(credits_md_0),
        );
        assert_eq!(actual, expected);
    }
}
